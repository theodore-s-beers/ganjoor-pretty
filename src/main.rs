#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::unused_async, clippy::multiple_crate_versions)]

use std::io::Write;
use std::path::Path;

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use scraper::{ElementRef, Html, Selector};
use tempfile::NamedTempFile;

use ganjoor_pretty::{collect_lines, generate_title, get_ganjoor, pandoc};

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
    HttpResponse::Ok().body("Hello world!")
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
    let full_path = path.into_inner();
    let ganjoor_url = format!("https://ganjoor.net/{}", full_path);

    // Get Ganjoor HTML
    let response_text = match get_ganjoor(&ganjoor_url) {
        Ok(text) => text,
        Err(_) => return HttpResponse::Ok().body("Something went wrong!"),
    };

    // Parse HTML
    let parsed = Html::parse_document(&response_text);

    // Set up selectors for scraping
    let title_selector = Selector::parse("#page-hierarchy a").unwrap();
    let line_selector = Selector::parse("div.b").unwrap();

    // Scrape title and poem lines
    let title_parts: Vec<ElementRef> = parsed.select(&title_selector).collect();
    let lines: Vec<ElementRef> = parsed.select(&line_selector).collect();

    // If we got no lines, abort
    if lines.is_empty() {
        return HttpResponse::Ok().body("Something went wrong!");
    }

    // Generate title and collect lines
    let title = generate_title(&title_parts);
    let collected = collect_lines(&lines);

    // Write lines to temp file
    let mut tempfile = match NamedTempFile::new() {
        Ok(file) => file,
        Err(_) => return HttpResponse::Ok().body("Something went wrong!"),
    };

    match write!(tempfile, "{}", collected) {
        Ok(_) => (),
        Err(_) => return HttpResponse::Ok().body("Something went wrong!"),
    }

    // Run temp file through Pandoc
    let output_text = match pandoc(tempfile.path(), &title) {
        Ok(output) => output,
        Err(_) => return HttpResponse::Ok().body("Something went wrong!"),
    };

    HttpResponse::Ok().body(output_text)
}
