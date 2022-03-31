#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::unused_async, clippy::multiple_crate_versions)]

use std::io::Write;
use std::process::Command;
use std::str;

use actix_web::{get, http::StatusCode, web, App, HttpResponse, HttpServer, Responder};
use anyhow::anyhow;
use isahc::prelude::*;
use scraper::{ElementRef, Html, Selector};
use tempfile::NamedTempFile;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(index))
        .bind(("127.0.0.1", 5779))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{full_path:.+}")]
async fn index(path: web::Path<String>) -> impl Responder {
    let full_path = path.into_inner();
    let ganjoor_url = format!("https://ganjoor.net/{}", full_path);

    let response_text = match get_ganjoor(&ganjoor_url) {
        Ok(text) => text,
        Err(_) => return HttpResponse::Ok().body("Something went wrong!"),
    };

    let parsed = Html::parse_document(&response_text);

    let title_selector = Selector::parse("#page-hierarchy a").unwrap();
    let line_selector = Selector::parse("div.b").unwrap();

    let title_parts: Vec<ElementRef> = parsed.select(&title_selector).collect();
    let lines: Vec<ElementRef> = parsed.select(&line_selector).collect();

    if lines.is_empty() {
        return HttpResponse::Ok().body("Something went wrong!");
    }

    let mut title = String::from("title=");

    for (i, element) in title_parts.iter().enumerate() {
        title.push_str(element.inner_html().trim());
        if i < (title_parts.len() - 1) {
            title.push_str(" » ");
        }
    }

    let mut collected = String::new();

    for element in lines {
        collected.push_str(&element.html());
    }

    let mut tempfile = match NamedTempFile::new() {
        Ok(file) => file,
        Err(_) => return HttpResponse::Ok().body("Something went wrong!"),
    };

    match write!(tempfile, "{}", collected) {
        Ok(_) => (),
        Err(_) => return HttpResponse::Ok().body("Something went wrong!"),
    }

    let pandoc = match Command::new("pandoc")
        .arg(tempfile.path())
        .args([
            "-f",
            "html",
            "-t",
            "html",
            "-s",
            "-M",
            "document-css=false",
            "-M",
            &title,
            "-M",
            "lang=ar",
            "-M",
            "dir=rtl",
            "-H",
            "head.html",
            "-A",
            "script.html",
        ])
        .output()
    {
        Ok(output) => output,
        Err(_) => return HttpResponse::Ok().body("Something went wrong!"),
    };

    let ultima = match str::from_utf8(&pandoc.stdout) {
        Ok(string) => string.to_owned(),
        Err(_) => return HttpResponse::Ok().body("Something went wrong!"),
    };

    HttpResponse::Ok().body(ultima)
}

fn get_ganjoor(ganjoor_url: &str) -> Result<String, anyhow::Error> {
    let mut response = isahc::get(ganjoor_url)?;

    let status = response.status();
    if status == StatusCode::NOT_FOUND {
        return Err(anyhow!("404"));
    }

    let response_text = response.text()?;
    Ok(response_text)
}
