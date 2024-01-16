use maud::{html, Markup};
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::extract::State;
use crate::SiteState;
use super::base;

pub async fn index(State(state): State<Arc<RwLock<SiteState>>>) -> Markup {
    let state = state.read().await;

   let things = state.things.clone();

    let content = html! {
        div class="pure-g hero section" {
            div class="pure-u-1" {
                h1 { "Things I've Made" }
                ul {
                    @for thing in things.clone() {
                        li { 
                            (thing.date.format("%Y-%m-%d").to_string()) ": "
                            a href=(thing.link) { (thing.title) }
                            @if let Some(description) = &thing.description {
                                " - "
                                (description)
                            }
                        };
                    }
                }
            }
        }

        div class="pure-g hero section centered" {
            "Go to " a href="/" { "home" } " | " a href="/posts/" { "words" }
        }
    };

    base("Home".to_owned(), content, state.clone())
}
