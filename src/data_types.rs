use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{PlayerTrackingData, rcon_manager::RCONManager};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub token: String,
    pub admin_role_id: u64,
    pub rcon: ConfigRCON,
    pub player_log_channel_id: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigRCON {
    pub address: String,
    pub port: String,
    pub password: String,
}

pub struct Data {
    pub config: Config,
    pub rcon: Arc<Mutex<RCONManager>>,
    pub player_tracker: Arc<Mutex<PlayerTrackingData>>,
}


pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
