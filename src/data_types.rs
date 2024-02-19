use std::sync::Arc;

use rcon::{AsyncStdStream, Connection};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::PlayerTrackingData;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub admin_role_id: u64,
    pub rcon: ConfigRcon,
    pub player_log_channel_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigRcon {
    pub address: String,
    pub port: String,
    pub password: String,
}

pub struct Data {
    pub config: Config,
    pub rcon: Arc<Mutex<Connection<AsyncStdStream>>>,
    pub player_tracker: Arc<Mutex<PlayerTrackingData>>,
}


pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
