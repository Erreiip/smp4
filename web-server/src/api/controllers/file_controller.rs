use std::fs;
use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use actix_web::{post, web, HttpResponse, Responder};
use actix_web::web::{Data, ServiceConfig};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::SystemTime;
use actix_web::http::StatusCode;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde_json::Number;
use crate::api::api_manager::AppState;
use crate::database::repositories::users_repository::UsersRepository;
use crate::services::jwt_service::JwtService;

#[derive(Debug, Deserialize)]
struct UploadFileMetadata {
    name: String,
    description: String,
}

#[derive(Debug, MultipartForm)]
struct UploadFileRequest {
    #[multipart(limit = "100MB")]
    file: TempFile,
    json: MpJson<UploadFileMetadata>,
}

#[derive(Debug, Deserialize)]
struct VerifyFileMetadata {
    name: String,
}

#[derive(Debug, MultipartForm)]
struct VerifyFileRequest {
    #[multipart(limit = "100MB")]
    file: TempFile,
    json: MpJson<VerifyFileMetadata>,
}

#[derive(Debug, Serialize)]
struct SMP4Metadata {
    date: SystemTime,
    author: String,
    oid: Number,
    description: String,
    email: String,
    license: String,
}

#[post("")]
async fn post_file(state: Data<AppState>, MultipartForm(form): MultipartForm<UploadFileRequest>, credentials: BearerAuth) -> impl Responder {
    let check_name = form.json.name.clone();
    if !check_name.ends_with(".mp4") {
        return HttpResponse::new(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    if form.file.size == 0 {
        return HttpResponse::new(StatusCode::NOT_ACCEPTABLE);
    }

    match build_smp4metadata(state, credentials, form.json.description.clone()).await {
        Ok(smp4_metadata) => match build_smp4video(form, smp4_metadata) {
            Ok((new_filename, smp4video)) => HttpResponse::Ok()
                .content_type("application/octet-stream")
                .insert_header(("Content-Disposition", format!("attachment; filename=\"{}\"", new_filename)))
                .body(smp4video),
            Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(_) => {
            log::error!("Failed to build SMP video metadata");
            HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn build_smp4metadata(state: Data<AppState>, credentials: BearerAuth, description: String) -> Result<SMP4Metadata, ()> {
    let token = credentials.token();
    let jwt_service = JwtService::new();

    match jwt_service.get_claims_from_token(String::from(token)) {
        Ok(claims) => {
            let repo = UsersRepository::new(state.db.clone());

            match repo.get_user_by_id(claims.id).await {
                Ok(user) => {
                    Ok(SMP4Metadata {
                        date: SystemTime::now(),
                        author: user.username,
                        oid: Number::from(1),
                        description,
                        email: user.email,
                        license: String::from("tobedetermined"),
                    })
                },
                Err(e) => {
                    log::warn!("Failed to get user: {}", e);
                    Err(())
                }
            }
        }
        Err(_) => Err(())
    }
}

fn build_smp4video(form: UploadFileRequest, smp4_metadata: SMP4Metadata) -> Result<(String, Vec<u8>), ()> {
    log::info!("Used metadata: {:?}", smp4_metadata);

    let original_path = Path::new(&form.json.name);
    if let Err(e) = fs::copy(form.file.file.path(), original_path) {
        log::info!("Error during file saving : {}", e);
        return Err(());
    }

    let new_filename = form.json.name.replace(".mp4", ".smp4");
    let new_path = Path::new(&new_filename);

    if let Err(e) = fs::copy(original_path, new_path) {
        log::info!("Error during file copy : {}", e);
        return Err(());
    }

    match fs::read(new_path) {
        Ok(content) => Ok((new_filename, content)),
        Err(e) => {
            log::info!("Error during file reading : {}", e);
            Err(())
        },
    }
}

#[post("/verify")]
async fn verify_file(_state: Data<AppState>, MultipartForm(form): MultipartForm<VerifyFileRequest>) -> impl Responder {
    let check_name = form.json.name.clone();
    if !check_name.ends_with(".smp4") {
        return HttpResponse::new(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    if form.file.size == 0 {
        return HttpResponse::new(StatusCode::NOT_ACCEPTABLE);
    }

    let response_body = SMP4Metadata {
        date: SystemTime::now(),
        author: String::from("Paul Helleu"),
        oid: Number::from(1),
        description: String::from("Ceci est une description"),
        email: String::from("iamabogoss@gmail.com"),
        license: String::from("tobedetermined"),
    };

    HttpResponse::Ok().json(response_body)
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("file")
            .service(post_file)
            .service(verify_file)
    );
}