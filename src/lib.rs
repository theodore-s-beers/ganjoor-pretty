use std::path::Path;
use std::process::Command;
use std::str;

use isahc::{prelude::*, Request};

pub fn get_ganjoor(ganjoor_url: &str) -> Result<String, anyhow::Error> {
    let mut response = Request::get(ganjoor_url)
        .header("Accept", "application/json")
        .body(())?
        .send()?;

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
