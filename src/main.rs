use axum::{
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};

use axum_extra::extract::cookie::CookieJar;

use log::{error, info};
use std::sync::Arc;
use tokio::sync::RwLock;
use maud::{Markup, html};

use tower_http::services::ServeDir;
mod site;
mod things;
mod update;
mod utils;
mod words;

async fn health() -> Markup {
    html! {
        "Ok"
        br;
        "Build Info: " (std::env::var("TIME").unwrap_or_else(|_| String::from("Unknown")))
        br;
        "Ref: " (std::env::var("REF").unwrap_or_else(|_| String::from("Unknown")))
        br;
        "Commit: " (std::env::var("COMMIT").unwrap_or_else(|_| String::from("Unknown")))
    }
}

#[derive(Debug, Clone)]
pub struct ClientState {
    pub theme: String,
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

    let things =
        things::read_things_from_file("./content/things.csv").expect("Failed to read things");

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
        .nest_service("/assets", ServeDir::new("./assets"))
        .route("/health", get(health))
        .route("/posts/", get(site::words::index))
        .route("/posts/:slug", get(site::words::post))
        .route("/things/", get(site::things::index))
        .route("/", get(site::home::home))
        .with_state(state)
        .layer(middleware::from_fn(middleware_apply_client_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn middleware_apply_client_state(
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    let mut state = ClientState {
        theme: String::from("DEFAULT"),
    };

    if let Some(cookie) = jar.get("colorscheme") {
        state.theme = cookie.value().to_string();
    }

    request.extensions_mut().insert(state);

    let response = next.run(request).await;

    response
}
