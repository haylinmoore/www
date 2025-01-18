use axum::{
    extract::{Request, State},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};

use axum_extra::extract::cookie::CookieJar;

use log::{error, info};
use maud::{html, Markup};
use std::sync::Arc;
use tokio::sync::RwLock;

use tower_http::services::ServeDir;
mod badges;
mod name;
mod rss;
mod site;
mod sitemap;
mod things;
mod update;
mod utils;
mod webring;
mod words;

async fn health(State(state): State<Arc<RwLock<SiteState>>>) -> Markup {
    let state = state.read().await;

    html! {
       "Ok"
        br;
        "Build Info: " (std::env::var("TIME").unwrap_or_else(|_| String::from("Unknown")))
        br;
        "Ref: " (std::env::var("REF").unwrap_or_else(|_| String::from("Unknown")))
        br;
        "Commit: " (std::env::var("COMMIT").unwrap_or_else(|_| String::from("Unknown")))
        br;
        "Last Updated: " (state.last_updated)
        br;
        "Name: " (state.name.uppercase_full_str())
        br;
        "Webring: " (if state.webring.is_some() { format!(
            "{} <- {} -> {}",
            state.webring.as_ref().unwrap().prev.name,
            state.webring.as_ref().unwrap().member.name,
            state.webring.as_ref().unwrap().next.name,
        )
        } else { "None".to_string() }
        )
    }
}

#[derive(Debug, Clone)]
pub struct ClientState {
    pub theme: String,
}

#[derive(Clone)]
pub struct SiteState {
    name: name::Name,
    last_updated: String,
    things: Vec<things::Thing>,
    words: Vec<words::Post>,
    sitemap: Vec<u8>,
    badges: Vec<badges::Badge>,
    webring: Option<webring::MemberGetResponse>,
    build_info: utils::BuildInfo,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting up!");

    let things =
        things::read_things_from_file("./content/things.csv").expect("Failed to read things");

    let words = words::init("./content/words/");

    let mut state = SiteState {
        name: name::Name::Haylin,
        last_updated: String::from(""),
        things,
        words,
        sitemap: vec![],
        badges: vec![],
        webring: None,
        build_info: utils::build_info(),
    };

    state.webring = webring::get_webring_link().await;

    state.sitemap = sitemap::init(state.clone()).expect("Failed to init sitemap");

    state.badges =
        badges::read_badges_from_file("./content/badges.csv").expect("Failed to read badges");

    let state = Arc::new(RwLock::new(state));

    let cloned_state = Arc::clone(&state);

    update::update(cloned_state.clone()).await.unwrap();

    tokio::spawn(async move {
        loop {
            if let Err(e) = update::update(cloned_state.clone()).await {
                error!("Error updating: {}", e);
            };

            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    });

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("./assets"))
        .route("/88x31.png", get(badges::my88x31))
        .route("/health", get(health))
        .route("/posts/", get(site::words::index))
        .route("/posts/:slug", get(site::words::post))
        .route("/posts/:slug/", get(site::words::post))
        .route("/things/", get(site::things::index))
        .route("/", get(site::home::home))
        .route("/sitemap.xml", get(sitemap::get))
        .route("/index.nginx-debian.html", get(site::nginx::get))
        .route("/feed.xml", get(rss::get))
        .fallback(get(site::error404::e404))
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
        theme: String::from("white"),
    };

    if let Some(cookie) = jar.get("colorscheme") {
        state.theme = cookie.value().to_string();
    }

    request.extensions_mut().insert(state);

    next.run(request).await
}
