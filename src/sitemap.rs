use crate::SiteState;
use anyhow::Result;
use axum::{body, extract::State, response::Response};
use sitemap;
use std::sync::Arc;
use tokio::sync::RwLock;

const BASE_URL: &str = "https://hamptonmoore.com";

pub fn init(state: SiteState) -> Result<Vec<u8>> {
    let mut sm: Vec<u8> = Vec::new();
    let smw = sitemap::writer::SiteMapWriter::new(&mut sm);
    let mut urlwriter = smw.start_urlset()?;
    urlwriter.url(BASE_URL)?;
    let static_pages = vec!["things", "posts"];
    for page in static_pages {
        urlwriter.url(format!("{}/{}/", BASE_URL, page))?;
    }
    for project in state.words {
        urlwriter.url(format!("{}/posts/{}", BASE_URL, project.slug))?;
    }
    urlwriter.end()?;
    Ok(sm)
}

pub async fn get(State(state): State<Arc<RwLock<SiteState>>>) -> Response {
    let state = state.read().await;

    Response::builder()
        .header("Content-Type", "application/xml")
        .body(body::Body::from(state.sitemap.clone()))
        .unwrap()
}
