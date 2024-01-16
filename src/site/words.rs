use super::base;
use crate::{ClientState, SiteState};
use axum::extract::{Extension, Path, State};
use maud::{html, Markup, PreEscaped};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::words::get;

pub async fn index(
    State(state): State<Arc<RwLock<SiteState>>>,
    Extension(client_state): Extension<ClientState>,
) -> Markup {
    let state = state.read().await;

    let words = state.words.clone();

    let content = html! {
        div class="pure-g hero section" {
            div class="pure-u-1" {
                h1 { "Words I've Written" }
                ul {
                    @for post in words.clone() {
                        li {
                            (post.date.format("%Y-%m-%d").to_string()) ": "
                            a href=(post.link) { (post.title) }
                            (" - ")
                            (post.description)
                        };
                    }
                }
            }
        }

        div class="pure-g hero section centered" {
            "Go to " a href="/" { "home" } " | " a href="/things/" { "things" }
        }
    };

    base("Posts".to_owned(), content, state.clone(), client_state)
}

pub async fn post(
    State(state): State<Arc<RwLock<SiteState>>>,
    Path(slug): Path<String>,
    Extension(client_state): Extension<ClientState>,
) -> Markup {
    let state = state.read().await;
    let words = state.words.clone();

    let post = get(words, &slug).unwrap();

    let content = html! {
        div class="pure-g hero section" {
            div class="pure-u-1" {
                h1 { (post.title) }
                p { (post.date.format("%Y-%m-%d").to_string()) " - " (post.description) }
                hr {}
                div {
                    (PreEscaped(post.body))
                }
            }
        }

        div class="pure-g hero section centered" {
            "Go to " a href="/" { "home" } " | " a href="/posts/" { "words" }
        }
    };

    base(post.title, content, state.clone(), client_state)
}
