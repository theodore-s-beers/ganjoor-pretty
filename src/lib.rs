use std::path::Path;
use std::process::Command;
use std::str;

use isahc::prelude::*;
use scraper::ElementRef;

pub fn collect_lines(lines: &[ElementRef]) -> String {
    let mut collected = String::new();

    for element in lines {
        collected.push_str(&element.html());
    }

    collected
}

pub fn generate_title(title_parts: &[ElementRef]) -> String {
    let mut title = String::from("title=");

    for (i, element) in title_parts.iter().enumerate() {
        title.push_str(element.inner_html().trim());
        if i < (title_parts.len() - 1) {
            title.push_str(" » ");
        }
    }

    title
}

pub fn get_ganjoor(ganjoor_url: &str) -> Result<String, anyhow::Error> {
    let mut response = isahc::get(ganjoor_url)?;

    let response_text = response.text()?;

    Ok(response_text)
}

pub fn pandoc(path: &Path, title: &str) -> Result<String, anyhow::Error> {
    let output = Command::new("pandoc")
        .arg(path)
        .args([
            "-f",
            "html",
            "-t",
            "html",
            "-s",
            "-M",
            "document-css=false",
            "-M",
            title,
            "-M",
            "lang=ar",
            "-M",
            "dir=rtl",
            "-H",
            "head.html",
            "-A",
            "script.html",
        ])
        .output()?;

    let output_text = str::from_utf8(&output.stdout)?.to_owned();

    Ok(output_text)
}
