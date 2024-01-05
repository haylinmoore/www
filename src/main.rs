use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, error};
use axum::{
    response::Html,
    routing::{get, get_service},
    Router
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
mod site;
mod update;

async fn health() -> Html<String> {
    Html(String::from("OK"))
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Steam {
    persona_state: String,
    persona_name: String,
    profile_url: String,
    avatar: String,
    last_logoff: String,
    is_gaming: bool,
    game_extra_info: String,
    game_url: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Discord {
    status_desk: String,
    status_web: String,
    status_mobile: String,
    custom_status: String,
    status_emoji: String,
    updated_at: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Cloud {
    is_down: bool,
    down_since: String,
}

#[derive(Clone)]
pub struct SiteState {
    workstation: String,
    last_updated: String,
    steam: Steam,
    discord: Discord,
    cloud: Cloud,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting up!");

    let state = Arc::new(RwLock::new(SiteState {
        workstation: String::from(""),
        last_updated: String::from(""),
        steam: Steam {
            persona_state: String::from(""),
            persona_name: String::from(""),
            profile_url: String::from(""),
            avatar: String::from(""),
            last_logoff: String::from(""),
            is_gaming: false,
            game_extra_info: String::from(""),
            game_url: String::from(""),
        },
        discord: Discord {
            status_desk: String::from(""),
            status_web: String::from(""),
            status_mobile: String::from(""),
            custom_status: String::from(""),
            status_emoji: String::from(""),
            updated_at: String::from(""),
        },
        cloud: Cloud {
            is_down: false,
            down_since: String::from(""),
        },
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
        .route("/", get(site::home::home))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().expect("Invalid address"))
        .serve(app.into_make_service())
        .await
        .expect("Server failed");
}

