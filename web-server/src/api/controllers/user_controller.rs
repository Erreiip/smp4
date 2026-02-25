use actix_web::{get, web, HttpResponse, Responder};
use actix_web::web::{Data, ServiceConfig};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::NaiveDateTime;
use serde::Serialize;
use crate::api::api_manager::AppState;
use crate::database::repositories::users_repository::UsersRepository;
use crate::services::jwt_service::JwtService;

#[derive(Serialize)]
struct GetUserResponse {
    username: String,
    email: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[get("")]
async fn get_user(state: Data<AppState>, credentials: BearerAuth) -> impl Responder {
    let token = credentials.token();
    let jwt_service = JwtService::new();

    match jwt_service.get_claims_from_token(String::from(token)) {
        Ok(claims) => {
            let repo = UsersRepository::new(state.db.clone());

            match repo.get_user_by_id(claims.id).await {
                Ok(user) => {
                    let response_body = GetUserResponse {
                        username: user.username, email: user.email,
                        created_at: user.created_at, updated_at: user.updated_at,
                    };
                    HttpResponse::Ok().json(response_body)
                },
                Err(e) => {
                    log::warn!("Failed to get user: {}", e);
                    HttpResponse::Unauthorized().await.unwrap()
                }
            }
        }
        Err(_) => HttpResponse::Unauthorized().await.unwrap()
    }
}
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("user")
            .service(get_user)
    );
}