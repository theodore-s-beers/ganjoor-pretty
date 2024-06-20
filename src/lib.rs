#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc)]

use std::path::Path;
use std::process::Command;
use std::str;

const API_PREFIX: &str = "https://api.ganjoor.net/api/ganjoor/poem?catInfo=false&catPoems=false&rhymes=false&recitations=false&images=false&songs=false&comments=false&verseDetails=false&navigation=false&url=/";

#[must_use]
pub fn construct_url(path: &str) -> String {
    format!("{API_PREFIX}{path}")
}

pub async fn get_ganjoor(ganjoor_url: &str) -> Result<String, anyhow::Error> {
    let response_text = reqwest::get(ganjoor_url).await?.text().await?;
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
            "static/head.html",
        ])
        .output()?;

    let output_text = str::from_utf8(&output.stdout)?.to_owned();
    Ok(output_text)
}
