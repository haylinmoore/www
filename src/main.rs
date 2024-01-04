use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    response::Html,
    routing::get,
    Router
};
mod site;
mod update;

async fn health() -> Html<String> {
    Html(String::from("OK"))
}

#[derive(Clone)]
pub struct SiteState {
    workstation: String,
    last_updated: String,
}

#[tokio::main]
async fn main() {
    println!("Loading state.");

    let state = Arc::new(RwLock::new(SiteState {
        workstation: String::from(""),
        last_updated: String::from(""),
    }));

    println!("Starting webserver!");

    let cloned_state = Arc::clone(&state);
    tokio::spawn(async move {
        loop {
            update::update(cloned_state.clone()).await;
            // wait 2 mins
            tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;
        }
    });

    let app = Router::new()
        .route("/health", get(health))
        .route("/", get(site::home::home))
        .with_state(state);


    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}

