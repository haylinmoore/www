use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::fs;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Clone, Debug)]
pub struct Badge {
    pub alt: String,
    pub link: Option<String>,
    pub src: String,
}

pub async fn my88x31() -> impl IntoResponse {
    // Define the path to your image
    let image_path = "./assets/img/badges/haylin_button_e2.png";

    // Try to read the file
    match fs::read(image_path) {
        Ok(image_data) => {
            // Return the image data with a proper content-type header
            Response::builder()
                .header("Content-Type", "image/png")
                .body(image_data.into())
                .unwrap()
        }
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}

pub fn read_badges_from_file(file_path: &str) -> io::Result<Vec<Badge>> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut badges = Vec::new();

    for line in reader.lines().skip(1) {
        // Skip the header line
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() < 2 {
            continue;
        }

        let alt = parts[0].to_string();
        let mut src = parts[1].to_string();
        let mut link = parts.get(2).map(|s| s.to_string());

        // Check if src starts with https, otherwise prepend /assets/img/badges/
        if !src.starts_with("https") {
            src = format!("/assets/img/badges/{}", src);
        }

        // If link is empty string, set it to None
        if link == Some("".to_string()) {
            link = None;
        }

        badges.push(Badge { alt, link, src });
    }

    Ok(badges)
}
