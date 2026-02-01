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
    pub player_count_channel: Option<ConfigPlayerCountChannel>,
    pub tasks: Vec<PeriodicTask>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PeriodicTask {
    pub name: String,
    pub cron_time: String,
    pub timeout_minutes: u64,
    pub commands: Vec<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigPlayerCountChannel {
    pub id: u64,
    pub name: String
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
