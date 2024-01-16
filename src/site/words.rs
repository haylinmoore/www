use maud::{html, Markup};
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::extract::State;
use crate::SiteState;
use super::base;

use crate::words::{get, Post};

pub async fn index(State(state): State<Arc<RwLock<SiteState>>>) -> Markup {
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
                            a href=(post.slug) { (post.title) }
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

    base("Home".to_owned(), content, state.clone())
}
