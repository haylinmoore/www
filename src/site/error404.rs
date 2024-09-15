use super::{base, PageContext};
use crate::{ClientState, SiteState};
use axum::extract::{Extension, State};
use axum::{http::StatusCode, response::IntoResponse};
use maud::{html, PreEscaped};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn e404(
    State(state): State<Arc<RwLock<SiteState>>>,
    Extension(client_state): Extension<ClientState>,
) -> impl IntoResponse {
    let state = state.read().await;

    let content = html! {
        style type="text/css" {
            (r#"
            .reverse {
                -moz-transform: scale(-1, 1);
                -webkit-transform: scale(-1, 1);
                -o-transform: scale(-1, 1);
                -ms-transform: scale(-1, 1);
                transform: scale(-1, 1);
                }
            "#
            )
        }
        div class="pure-g hero section centered" {
            div class="pure-u-1" {
                h1 id="404" { "🐈" }
            }
        }

        div class="pure-g hero section centered" {
            "Go to " a href="/" { "home" }
        }

        script {
            (PreEscaped(r#"
                var friend = document.getElementById('404');
                setTimeout(()=>friend.classList.add("reverse"), 1500);
                setTimeout(()=>{friend.classList.remove("reverse"); friend.innerHTML = "😿 404, sorry page not found"}, 3000);
            "#
            ))
        }
    };

    (
        StatusCode::NOT_FOUND,
        base(
            PageContext {
                title: "404".to_string(),
                canonical: "/".to_string(),
            },
            content,
            state.clone(),
            client_state,
        ),
    )
}
