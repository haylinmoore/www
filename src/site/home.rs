use maud::{html, Markup};
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::extract::State;
use crate::SiteState;
use super::base;

pub async fn home(State(state): State<Arc<RwLock<SiteState>>>) -> Markup {
    let state = state.read().await;
    let workstation = state.workstation.clone();
    let last_updated = state.last_updated.clone();

    let content = html! {
        pre { (workstation) }
        pre { (last_updated) }
    };

    base(content, state.clone())
}
