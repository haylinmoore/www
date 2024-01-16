use super::base;
use crate::{ClientState, SiteState};
use axum::extract::{Extension, State};
use maud::{html, Markup};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn index(
    State(state): State<Arc<RwLock<SiteState>>>,
    Extension(client_state): Extension<ClientState>,
) -> Markup {
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

    base(
        "Things".to_owned(),
        content,
        state.clone(),
        client_state,
    )
}
