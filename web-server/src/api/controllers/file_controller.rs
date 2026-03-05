use std::collections::HashMap;
use crate::api::api_manager::AppState;
use crate::database::repositories::users_repository::UsersRepository;
use crate::services::jwt_service::JwtService;
use actix_multipart::form::{MultipartForm, json::Json as MpJson, tempfile::TempFile};
use actix_web::http::StatusCode;
use actix_web::web::{Data, ServiceConfig};
use actix_web::{HttpResponse, Responder, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use smp4_common::hash::hash_enum::hash_enum::SHA3;
use smp4_common::metadata::metadata::MetadataFields;
use smp4_common::sign::sign_enum::sign_enum::DILITHIUM2;
use smp4_common::sfile::sfile::{build_sfile, sfile_metadata, sfile_verify, truncate_sfile};

#[derive(Clone, Debug, Deserialize)]
struct UploadFileMetadata {
    name: String,
    description: String,
    link: Option<String>,
    licence: Option<String>,
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

#[derive(Debug, Deserialize)]
struct DecodeFileMetadata {
    name: String,
}

#[derive(Debug, MultipartForm)]
struct DecodeFileRequest {
    #[multipart(limit = "100MB")]
    file: TempFile,
    json: MpJson<DecodeFileMetadata>,
}

#[derive(Debug, Serialize)]
struct SMP4Metadata {
    date: SystemTime,
    author: String,
    oid: String,
    description: String,
    email: String,
    link: String,
    license: String,
}

#[post("")]
async fn post_file(
    state: Data<AppState>,
    MultipartForm(form): MultipartForm<UploadFileRequest>,
    credentials: BearerAuth,
) -> impl Responder {
    if form.file.size == 0 {
        return HttpResponse::new(StatusCode::NOT_ACCEPTABLE);
    }

    match build_smp4metadata(state, credentials, form.json.clone()).await {
        Ok(smp4_metadata) => match build_smp4video(form, smp4_metadata) {
            Ok((new_filename, smp4video)) => HttpResponse::Ok()
                .content_type("application/octet-stream")
                .insert_header((
                    "Content-Disposition",
                    format!("attachment; filename=\"{}\"", new_filename),
                ))
                .body(smp4video),
            Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => {
            log::error!("Failed to build SMP video metadata");
            HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn build_smp4metadata(
    state: Data<AppState>,
    credentials: BearerAuth,
    json_metadata: UploadFileMetadata,
) -> Result<HashMap<String, String>, ()> {
    let token = credentials.token();
    let jwt_service = JwtService::new();

    match jwt_service.get_claims_from_token(String::from(token)) {
        Ok(claims) => {
            let repo = UsersRepository::new(state.db.clone());

            match repo.get_user_by_id(claims.id).await {
                Ok(user) => {
                    let datetime: DateTime<Utc> = SystemTime::now().into();

                    let mut metadata: HashMap<String, String> = HashMap::new();
                    metadata.insert(MetadataFields::AUTHOR.to_string(), user.username);
                    metadata.insert(MetadataFields::DATE.to_string(), datetime.to_string());
                    metadata.insert(MetadataFields::OID.to_string(), "d023957e-37dd-449e-b324-8a3e499b5c46".to_string());
                    metadata.insert(MetadataFields::DESCRIPTION.to_string(), json_metadata.description);
                    metadata.insert(MetadataFields::EMAIL.to_string(), user.email);

                    if let Some(licence) = json_metadata.licence {
                        metadata.insert(MetadataFields::LICENSE.to_string(), licence);
                    }

                    if let Some(link) = json_metadata.link {
                        metadata.insert(MetadataFields::LINK_ORIGIN.to_string(), link);
                    }

                    Ok(metadata)
                }
                Err(e) => {
                    log::warn!("Failed to get user: {}", e);
                    Err(())
                }
            }
        }
        Err(_) => Err(()),
    }
}

fn build_smp4video(
    form: UploadFileRequest,
    smp4_metadata: HashMap<String, String>,
) -> Result<(String, Vec<u8>), ()> {
    log::info!("Used metadata: {:?}", smp4_metadata);

    let original_path = Path::new(&form.json.name);
    if let Err(e) = fs::copy(form.file.file.path(), original_path) {
        log::info!("Error during file saving : {}", e);
        return Err(());
    }

    match build_sfile(form.json.name.clone(), smp4_metadata, SHA3, DILITHIUM2) {
        Ok(path_buf) => {
            let file_name = path_buf.clone().file_name().unwrap().to_str().unwrap().to_string();

            match fs::read(path_buf) {
                Ok(content) => Ok((file_name, content)),
                Err(e) => {
                    log::info!("Error during file reading : {}", e);
                    Err(())
                }
            }
        },
        Err(e) => {
            log::error!("Error during sfile build : {}", e);
            Err(())
        }
    }
}

#[post("/verify")]
async fn verify_file(
    _state: Data<AppState>,
    MultipartForm(form): MultipartForm<VerifyFileRequest>,
) -> impl Responder {
    if form.file.size == 0 {
        return HttpResponse::new(StatusCode::NOT_ACCEPTABLE);
    }

    let original_path = Path::new(&form.json.name);
    if let Err(e) = fs::copy(form.file.file.path(), original_path) {
        log::info!("Error during file saving : {}", e);
        return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR);
    }

    match sfile_verify(form.json.name.clone()) {
        Ok(result) => {
            if !result {
                return HttpResponse::new(StatusCode::NOT_ACCEPTABLE);
            }

            match retrieve_file_metadata(form.json.name.clone()) {
                Ok(metadata) => HttpResponse::Ok().json(metadata),
                Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(e) => {
            log::error!("Error during sfile verification : {}", e);
            HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[post("/decode")]
async fn decode_file(
    _state: Data<AppState>,
    MultipartForm(form): MultipartForm<DecodeFileRequest>,
) -> impl Responder {
    if form.file.size == 0 {
        return HttpResponse::new(StatusCode::NOT_ACCEPTABLE);
    }

    let original_path = Path::new(&form.json.name);
    if let Err(e) = fs::copy(form.file.file.path(), original_path) {
        log::info!("Error during file saving : {}", e);
        return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR);
    }

    match sfile_verify(form.json.name.clone()) {
        Ok(result) => {
            if !result {
                return HttpResponse::new(StatusCode::NOT_ACCEPTABLE);
            }

            match retrieve_file_content(form.json.name.clone()) {
                Ok((new_filename, file)) => {
                    HttpResponse::Ok()
                        .content_type("application/octet-stream")
                        .insert_header((
                            "Content-Disposition",
                            format!("attachment; filename=\"{}\"", new_filename),
                        ))
                        .body(file)
                }
                Err(_) => {
                    HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        },
        Err(e) => {
            log::error!("Error during sfile verification : {}", e);
            HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn retrieve_file_metadata(sfile_path: String) -> Result<SMP4Metadata, ()> {
    match sfile_metadata(sfile_path) {
        Ok(hm_metadata) => {
            let date_str = hm_metadata.get(MetadataFields::DATE).cloned().unwrap_or(String::from(""));
            let timestamp = date_str.parse::<i64>().unwrap_or(0);
            let datetime: DateTime<Utc> = DateTime::<Utc>::from_timestamp(timestamp, 0).expect("invalid Timestamp");

            Ok(SMP4Metadata {
                date: SystemTime::from(datetime),
                author: hm_metadata.get(MetadataFields::AUTHOR).cloned().unwrap_or(String::from("")),
                oid: hm_metadata.get(MetadataFields::OID).cloned().unwrap_or(String::from("1")),
                description: hm_metadata.get(MetadataFields::DESCRIPTION).cloned().unwrap_or(String::from("")),
                email: hm_metadata.get(MetadataFields::EMAIL).cloned().unwrap_or(String::from("")),
                link: hm_metadata.get(MetadataFields::LINK_ORIGIN).cloned().unwrap_or(String::from("")),
                license: hm_metadata.get(MetadataFields::LICENSE).cloned().unwrap_or(String::from("")),
            })
        },
        Err(e) => {
            log::error!("Error during metadata retrieving : {}", e);
            Err(())
        }
    }
}

fn retrieve_file_content(sfile_path: String) -> Result<(String, Vec<u8>), ()> {
    match truncate_sfile(sfile_path) {
        Ok(path_buf) => {
            let file_name = path_buf.clone().file_name().unwrap().to_str().unwrap().to_string();

            match fs::read(path_buf) {
                Ok(content) => Ok((file_name, content)),
                Err(e) => {
                    log::info!("Error during file reading : {}", e);
                    Err(())
                }
            }
        }
        Err(e) => {
            log::info!("Error during file truncation : {}", e);
            Err(())
        }
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("file")
        .service(post_file)
        .service(verify_file)
        .service(decode_file));
}
