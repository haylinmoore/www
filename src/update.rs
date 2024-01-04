use std::sync::Arc;
use tokio::sync::RwLock;
use crate::SiteState;
use color_eyre::eyre::Result;

pub async fn update(state: Arc<RwLock<SiteState>>) -> Result<()> {
    let mut state = state.write().await;

    let workstation_text = reqwest::get("http://api.ezri.pet/ide0.text").await?.text().await?;
    state.workstation = workstation_text;

    let steam_json = reqwest::get("http://api.ezri.pet/steam").await?.text().await?;
    state.steam = serde_json::from_str(&steam_json)?;

    let discord_json = reqwest::get("http://api.ezri.pet/discord").await?.text().await?;
    state.discord = serde_json::from_str(&discord_json)?;

    let cloud_json = reqwest::get("http://api.ezri.pet/ezricloud").await?.text().await?;
    state.cloud = serde_json::from_str(&cloud_json)?;

    let last_updated_text = String::from(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string().as_str());
    state.last_updated = last_updated_text;

    Ok(())
}
