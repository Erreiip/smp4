use actix_web::{post, web::{Data, Json, ServiceConfig}, Responder, HttpResponse, web};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::{Hasher, Verifier};
use serde::{Deserialize, Serialize};
use log::error;
use crate::api::api_manager::{AppState};
use crate::database::repositories::users_repository::UsersRepository;
use crate::services::jwt_service::JwtService;

#[derive(Deserialize)]
struct CreateUserRequest {
	username: String,
	email: String,
	password: String,
}

#[derive(Serialize)]
struct ErrorResponse {
	error: String,
}

#[derive(Serialize)]
struct RetrieveTokenResponse {
	token: String,
}

#[post("/register")]
async fn register(state: Data<AppState>, body: Json<CreateUserRequest>) -> impl Responder {
	let user: CreateUserRequest = body.into_inner();
	
	let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set");
	let mut hasher = Hasher::default();
	let hash = hasher
		.with_password(user.password)
		.with_secret_key(hash_secret)
		.hash()
		.unwrap();

	let repo = UsersRepository::new(state.db.clone());
	match repo.create_user(user.username, user.email, hash).await {
		Ok(user_created) => {
			let jwt_service = JwtService::new();
			let token = jwt_service.generate_token_from_user(user_created);
			let response_body = RetrieveTokenResponse { token };
			HttpResponse::Ok().json(response_body)
		},
		Err(error) => {
			match error.as_database_error() {
				Some(db_error) => {
					if db_error.is_unique_violation() {
						let response_body = ErrorResponse { error : String::from("Username or email already used") };
						return HttpResponse::BadRequest().json(response_body);
					}
				}
				_ => ()
			}

			error!("Failed to create user: {}", error);
			HttpResponse::InternalServerError().await.unwrap()
		},
	}
}

#[post("/login")]
async fn login(state: Data<AppState>, credentials: BasicAuth) -> impl Responder {
	let username = credentials.user_id();
	let password = credentials.password();

	match password {
		None => HttpResponse::Unauthorized().await.unwrap(),
		Some(pass) => {
			let repo = UsersRepository::new(state.db.clone());

			match repo.get_user_by_username(username.to_string()).await {
				Ok(user) => {
					let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set");
					let mut verifier = Verifier::default();
					let is_valid = verifier
						.with_hash(user.clone().password)
						.with_password(pass)
						.with_secret_key(hash_secret)
						.verify()
						.unwrap();
					
					if is_valid {
						let jwt_service = JwtService::new();
						let token = jwt_service.generate_token_from_user(user);
						let response_body = RetrieveTokenResponse { token };
						HttpResponse::Ok().json(response_body)
					} else {
						HttpResponse::Unauthorized().await.unwrap()
					}
				}
				Err(_) => HttpResponse::Unauthorized().await.unwrap()
			}
		}
	}
}

pub fn config(cfg: &mut ServiceConfig) {
	cfg.service(
		web::scope("auth")
			.service(register)
			.service(login)
	);
}