mod api;
mod database;
mod services;

use crate::api::api_manager;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	env_logger::init();

    api_manager::start().await
}
