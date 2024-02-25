use std::{sync::Arc, time::Duration};

use poise::serenity_prelude::{Cache, Http, Context, ActivityData, ChannelId, EditChannel};
use serde_json::json;
use tokio::sync::Mutex;

use crate::{rcon_manager::RCONManager, data_types::Config};

pub struct PlayerTrackingData {
    pub previous_player_list: Vec<String>,
    pub first: bool,
}

pub async fn check_players(
    rcon: Arc<Mutex<RCONManager>>,
    player_tracker: Arc<Mutex<PlayerTrackingData>>,
    // channel: &ChannelId,
    config: &Config,
    cache: Arc<Cache>,
    http: Arc<Http>,
    ctx: Context
) {
    loop {
        let mut tracker = player_tracker.lock().await;

        let players = rcon.lock().await.cmd("players").await;
        let mut player_list: Vec<String> = players.split('\n').map(|s| s.to_string()).collect();
        player_list.remove(0);
        player_list.sort();

        let mut prev_player_list = tracker.previous_player_list.clone();

        player_list.retain(|player| player.trim() != "");
        prev_player_list.retain(|player| player.trim() != "");

        if tracker.first || *prev_player_list != player_list || rcon.lock().await.did_connection_fail() {
            let suffix = if player_list.len() != 1 { "s" } else { "" } ;
            let activity = format!("{} player{} online.", player_list.len(), suffix);

            ctx.set_activity(Some(ActivityData::custom(activity)));
            println!("Player count changed, status changed to show {} players", player_list.len());

            // if let Some(count_channel) = &config.player_count_channel {
            //     let id = ChannelId::new(count_channel.id);
            //
            //     let name = count_channel.name.replace("$1", &player_list.len().to_string()).replace("$2", suffix);
            //     let map = json!({
            //         "name": name 
            //     });
            //     println!("Updating channel name to: {}", name);
            //
            //     let httpc = http.clone();
            //     tokio::spawn(async move { // rate limit bruh
            //         httpc.edit_channel(id, &map, None).await.unwrap();
            //         println!("Channel name updated");
            //     });
            // }

        }
        
        if !tracker.first && *prev_player_list != player_list {
            let id: ChannelId = ChannelId::new(config.player_log_channel_id);
            let channel = &cache.guilds()[0].channels(&http).await.unwrap()[&id];

            let mut joined_list = player_list.clone();

            for player in &prev_player_list {
                if let Ok(index) = joined_list.binary_search(player) {
                    joined_list.remove(index);
                }
            }

            for player in joined_list {
                channel.say((&cache, http.as_ref()), format!("**{} joined**", &player[1..]))
                    .await
                    .unwrap();
                println!("{} joined", &player[1..]);
            }

            for player in &prev_player_list {
                if let Err(_index) = player_list.binary_search(player) {
                    channel.say((&cache, http.as_ref()), format!("**{} left**", &player[1..]))
                        .await
                        .unwrap();
                    println!("{} left", &player[1..]);
                }
            }
        }

        tracker.previous_player_list = player_list;
        tracker.first = false;

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
