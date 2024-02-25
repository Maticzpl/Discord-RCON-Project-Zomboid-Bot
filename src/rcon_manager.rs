use std::time::Duration;

use poise::serenity_prelude::ActivityData;
use rcon::{AsyncStdStream, Connection};

use crate::data_types::ConfigRCON;

// Reconnects if smth goes wrong
pub struct RCONManager {
    pub connection: Connection<AsyncStdStream>,
    config: ConfigRCON,
    pub ctx: Option<poise::serenity_prelude::Context>,
    connection_did_fail: bool
}


impl RCONManager {
    pub async fn new(config: ConfigRCON) -> Self {
        loop {
            let res = <Connection<AsyncStdStream>>::builder()
                .connect(
                    format!("{}:{}", &config.address, &config.port),
                    &config.password,
                ).await;

            if let Ok(connection) = res {
                return RCONManager { 
                    connection,
                    config,
                    ctx: None,
                    connection_did_fail: false
                }    
            }
            println!("Connection not established, retrying...");
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    pub async fn cmd(&mut self, command: &str) -> String {
        self.connection_did_fail = false;
        loop {
            match self.connection.cmd(command).await {
                Ok(response) => {
                    return response;
                },
                Err(_) => {
                    self.connection_did_fail = true;
                    println!("Connection lost, reconnecting...");
                    if let Some(ctx) = &self.ctx {
                        ctx.set_activity(Some(ActivityData::custom("Server down")));
                    }
                    self.reconnect().await;
                    if let Some(ctx) = &self.ctx {
                        ctx.set_activity(Some(ActivityData::custom("Server running")));
                    }
                }
            }
        }

    }

    // Keep trying to connect every 5 seconds until connected
    async fn reconnect(&mut self) {
        loop {
            let res = <Connection<AsyncStdStream>>::builder()
                .connect(
                    format!("{}:{}", &self.config.address, &self.config.port),
                    &self.config.password,
                ).await;

            if let Ok(connection) = res {
                self.connection = connection;
                println!("Reconnect successful");
                return;
            }

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    pub fn did_connection_fail(&self) -> bool {
       self.connection_did_fail 
    }
    
}
