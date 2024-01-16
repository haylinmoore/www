use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, error};
use axum::{
    response::Html,
    routing::{get, get_service},
    Router
};

use tower_http::services::ServeDir;
mod site;
mod update;
mod utils;
mod things;
mod words;

async fn health() -> Html<String> {
    Html(String::from("OK"))
}


#[derive(Clone)]
pub struct SiteState {
    last_updated: String,
    things: Vec<things::Thing>,
    words: Vec<words::Post>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting up!");

    let things = things::read_things_from_file("./content/things.csv").expect("Failed to read things");

    let words = words::init("./content/words/");

    let state = Arc::new(RwLock::new(SiteState {
        last_updated: String::from(""),
        things,
        words,
    }));

    let cloned_state = Arc::clone(&state);
    tokio::spawn(async move {
        loop {
            if let Err(e) = update::update(cloned_state.clone()).await {
                error!("Error updating: {}", e);
            };
            // wait 1 min
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    });

    let app = Router::new()
        .nest_service("/assets", get_service(ServeDir::new("./assets")))
        .route("/health", get(health))
        .route("/posts/", get(site::words::index))
        .route("/things/", get(site::things::index))
        .route("/", get(site::home::home))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().expect("Invalid address"))
        .serve(app.into_make_service())
        .await
        .expect("Server failed");
}

