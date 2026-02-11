use actix_cors::Cors;
use actix_web::{dev::ServiceRequest, error::Error, web::{self}, App, HttpMessage, HttpServer};
use actix_web_httpauth::{
	extractors::{
		bearer::{self, BearerAuth},
		AuthenticationError,
	},
	middleware::HttpAuthentication,
};
use crate::database::db_manager::{self, Database};
use crate::api::controllers::{auth_controller, user_controller};
use crate::services::jwt_service::JwtService;

pub struct AppState {
	pub db: Database,
}

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
	let token = credentials.token();
	let jwt_service = JwtService::new();
	
	match jwt_service.get_claims_from_token(String::from(token)) {
		Ok(claims) => {
			req.extensions_mut().insert(claims);
			Ok(req)
		}
		Err(_) => {
			let config = req.app_data::<bearer::Config>().cloned().unwrap_or_default().scope("");
			Err((AuthenticationError::from(config).into(), req))
		}
	}
}

pub async fn start() -> std::io::Result<()> {
	let expose_ip: String = std::env::var("EXPOSE_IP").expect("EXPOSE_IP must be set");
	let expose_port: String = std::env::var("EXPOSE_PORT").expect("EXPOSE_PORT must be set");
	let expose_port_int: u16 = expose_port.parse::<u16>().unwrap();

	let database = db_manager::connect().await;
    let app_data = web::Data::new(AppState {
		db: database,
    });

    HttpServer::new(move || {
		let bearer_middleware = HttpAuthentication::bearer(validator);
		let cors = Cors::permissive();
        App::new()
            .app_data(app_data.clone())
			.wrap(cors)
			.configure(auth_controller::config)
			.service(
				web::scope("")
					.wrap(bearer_middleware)
					.configure(user_controller::config)
			)
    }).bind((expose_ip, expose_port_int))?
        .run()
        .await
}