use std::sync::Arc;
use tokio::sync::RwLock;
use crate::SiteState;

pub async fn update(state: Arc<RwLock<SiteState>>) -> () {
    let mut state = state.write().await;

    let workstation_text = reqwest::get("http://localhost:8080/ide0.text").await.unwrap().text().await.unwrap();
    state.workstation = workstation_text;

    let last_updated_text = String::from("Last updated: ") + chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string().as_str();
    state.last_updated = last_updated_text;
}
