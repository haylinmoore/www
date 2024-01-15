use std::sync::Arc;
use tokio::sync::RwLock;
use crate::SiteState;
use color_eyre::eyre::Result;

pub async fn update(state: Arc<RwLock<SiteState>>) -> Result<()> {
    let mut state = state.write().await;

    let last_updated_text = String::from(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string().as_str());
    state.last_updated = last_updated_text;

    Ok(())
}
