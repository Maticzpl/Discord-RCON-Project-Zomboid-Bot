use std::{sync::Arc, time::Duration};

use poise::serenity_prelude::{ChannelId, Cache, Http};
use rcon::{Connection, AsyncStdStream};
use tokio::sync::Mutex;

pub struct PlayerTrackingData {
    pub previous_player_list: Vec<String>,
    pub first: bool,
}

pub async fn check_players(
    rcon: Arc<Mutex<Connection<AsyncStdStream>>>,
    player_tracker: Arc<Mutex<PlayerTrackingData>>,
    channel: &ChannelId,
    cache: Arc<Cache>,
    http: Arc<Http>,
) {
    loop {
        let mut tracker = player_tracker.lock().await;

        let players = rcon.lock().await.cmd("players").await.unwrap();
        let mut player_list: Vec<String> = players.split('\n').map(|s| s.to_string()).collect();
        player_list.remove(0);
        player_list.sort();

        let mut prev_player_list = tracker.previous_player_list.clone();

        player_list.retain(|player| player.trim() != "");
        prev_player_list.retain(|player| player.trim() != "");

        if !tracker.first && *prev_player_list != player_list {
            let channel = &cache.guilds()[0].channels(&http).await.unwrap()[channel];

            let mut joined_list = player_list.clone();

            for player in &prev_player_list {
                if let Ok(index) = joined_list.binary_search(player) {
                    joined_list.remove(index);
                }
            }

            for player in joined_list {
                channel.say((&cache, http.as_ref()), format!("{} joined", &player[1..]))
                    .await
                    .unwrap();
            }

            for player in &prev_player_list {
                if let Err(_index) = player_list.binary_search(player) {
                    channel.say((&cache, http.as_ref()), format!("{} left", &player[1..]))
                        .await
                        .unwrap();
                }
            }
        }

        tracker.previous_player_list = player_list;
        tracker.first = false;

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
