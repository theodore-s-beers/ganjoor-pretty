#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, clippy::unused_async)]

use std::io::Write;
use std::path::Path;

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use tempfile::NamedTempFile;

use ganjoor_pretty::{construct_url, get_ganjoor, pandoc};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Poem {
    full_title: String,
    html_text: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(robots)
            .service(icon)
            .service(css)
            .service(js)
            .service(catchall)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/robots.txt")]
async fn robots() -> actix_web::Result<NamedFile> {
    let path = Path::new("static/robots.txt");
    Ok(NamedFile::open(path)?)
}

#[get("/icon.svg")]
async fn icon() -> actix_web::Result<NamedFile> {
    let path = Path::new("static/icon.svg");
    Ok(NamedFile::open(path)?)
}

#[get("/styles.css")]
async fn css() -> actix_web::Result<NamedFile> {
    let path = Path::new("static/styles.css");
    Ok(NamedFile::open(path)?)
}

#[get("/pretty.js")]
async fn js() -> actix_web::Result<NamedFile> {
    let path = Path::new("static/pretty.js");
    Ok(NamedFile::open(path)?)
}

#[get("/{full_path:.+}")]
async fn catchall(path: web::Path<String>) -> impl Responder {
    // Construct Ganjoor URL
    let ganjoor_url = construct_url(&path.into_inner());

    // Call Ganjoor API
    let Ok(response_text) = get_ganjoor(&ganjoor_url).await else {
        return HttpResponse::BadRequest().body(());
    };

    // Deserialize response
    let poem: Poem = match serde_json::from_str(&response_text) {
        Ok(obj) => obj,
        Err(_) => return HttpResponse::BadRequest().body(()),
    };

    let title = format!("title={}", poem.full_title);
    let text = poem.html_text;

    // Write lines to temp file
    let Ok(mut tempfile) = NamedTempFile::new() else {
        return HttpResponse::InternalServerError().body(());
    };

    if write!(tempfile, "{text}").is_err() {
        return HttpResponse::InternalServerError().body(());
    }

    // Run temp file through Pandoc
    let Ok(output_text) = pandoc(tempfile.path(), &title) else {
        return HttpResponse::InternalServerError().body(());
    };

    HttpResponse::Ok().body(output_text)
}
