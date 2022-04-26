#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::unused_async, clippy::multiple_crate_versions)]

use std::io::Write;
use std::path::Path;

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use tempfile::NamedTempFile;

use ganjoor_pretty::{get_ganjoor, pandoc};

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
            .service(css)
            .service(js)
            .service(catchall)
    })
    .bind(("127.0.0.1", 5779))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/robots.txt")]
async fn robots() -> actix_web::Result<NamedFile> {
    let path = Path::new("robots.txt");
    Ok(NamedFile::open(path)?)
}

#[get("/styles.css")]
async fn css() -> actix_web::Result<NamedFile> {
    let path = Path::new("styles.css");
    Ok(NamedFile::open(path)?)
}

#[get("/pretty.js")]
async fn js() -> actix_web::Result<NamedFile> {
    let path = Path::new("pretty.js");
    Ok(NamedFile::open(path)?)
}

#[get("/{full_path:.+}")]
async fn catchall(path: web::Path<String>) -> impl Responder {
    // Construct Ganjoor URL
    let poem_path = path.into_inner();

    let prefix = "https://api.ganjoor.net/api/ganjoor/poem?url=/";
    let suffix = "&catInfo=false&catPoems=false&rhymes=false&recitations=false&images=false&songs=false&comments=false&verseDetails=false&navigation=false";

    let ganjoor_url = format!("{}{}{}", prefix, poem_path, suffix);

    // Call Ganjoor API
    let response_text = match get_ganjoor(&ganjoor_url) {
        Ok(text) => text,
        Err(_) => return HttpResponse::BadRequest().body(()),
    };

    // Deserialize response
    let poem: Poem = match serde_json::from_str(&response_text) {
        Ok(obj) => obj,
        Err(_) => return HttpResponse::BadRequest().body(()),
    };

    let title = format!("title={}", poem.full_title);
    let text = poem.html_text;

    // Write lines to temp file
    let mut tempfile = match NamedTempFile::new() {
        Ok(file) => file,
        Err(_) => return HttpResponse::InternalServerError().body(()),
    };

    if write!(tempfile, "{}", text).is_err() {
        return HttpResponse::InternalServerError().body(());
    }

    // Run temp file through Pandoc
    let output_text = match pandoc(tempfile.path(), &title) {
        Ok(output) => output,
        Err(_) => return HttpResponse::InternalServerError().body(()),
    };

    HttpResponse::Ok().body(output_text)
}
