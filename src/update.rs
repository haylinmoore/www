use crate::SiteState;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::webring::get_webring_link;

pub async fn update(state: Arc<RwLock<SiteState>>) -> Result<()> {
    let last_updated_text = String::from(
        chrono::Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .as_str(),
    );

    let webring = get_webring_link().await;

    let mut state = state.write().await;
    state.last_updated = last_updated_text;
    state.webring = webring;

    Ok(())
}
