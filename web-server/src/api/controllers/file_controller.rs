use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use actix_web::{post, web, HttpResponse, Responder};
use actix_web::web::{Data, ServiceConfig};
use serde::Deserialize;
use std::io::Write;
use crate::api::api_manager::AppState;

#[derive(Debug, Deserialize)]
struct Metadata {
    name: String,
}

#[derive(Debug, MultipartForm)]
struct UploadFileRequest {
    #[multipart(limit = "100MB")]
    file: TempFile,
    json: MpJson<Metadata>,
}

#[post("")]
async fn post_file(_state: Data<AppState>, MultipartForm(form): MultipartForm<UploadFileRequest>) -> impl Responder {
    let check_name = form.json.name.clone();
    if !check_name.ends_with(".mp4") {
        return HttpResponse::BadRequest().body("Seuls les fichiers .mp4 sont acceptés.");
    }

    if form.file.size == 0 {
        return HttpResponse::BadRequest().body("Le fichier est vide.");
    }

    let mut response_file = tempfile::NamedTempFile::new().unwrap();
    writeln!(response_file, "Fichier reçu : {}", form.json.name).unwrap();
    writeln!(response_file, "Taille : {} octets", form.file.size).unwrap();

    let file_path = response_file.path().to_owned();
    let file_content = std::fs::read(file_path).unwrap();

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(file_content)
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("file")
            .service(post_file)
    );
}