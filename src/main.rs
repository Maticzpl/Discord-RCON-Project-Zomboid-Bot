mod data_types;
mod execute;
mod player_tracker;
mod rcon_manager;

use std::sync::Arc;

use data_types::Error;
use poise::serenity_prelude::{self as serenity, ChannelId};
use tokio::{fs, sync::Mutex};

use crate::data_types::{Config, Data};
use crate::execute::execute;
use crate::player_tracker::{PlayerTrackingData, check_players};
use crate::rcon_manager::RCONManager;


#[tokio::main]
async fn main() {
    let config: Config = serde_json::from_str(
        &fs::read_to_string("./config.json")
            .await
            .expect("Can't find config.json"),
    )
    .expect("Parsing config.json failed");

    println!("Connecting to RCON");
    let rcon = RCONManager::new(config.rcon.clone()).await;

    let rcon = Arc::new(Mutex::new(rcon));

    let player_tracker = Arc::new(Mutex::new(PlayerTrackingData {
        previous_player_list: vec![],
        first: true,
    }));

    let data: Data = Data {
        config,
        rcon: rcon.clone(),
        player_tracker,
    };

    let token = data.config.token.clone();
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![execute()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}



async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    #[allow(clippy::single_match)]
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Bot logged in as {}", data_about_bot.user.name);

            let cache = ctx.cache.clone();
            let http = ctx.http.clone();
            let ctx = ctx.clone(); // pray this is legal
            let rcon = data.rcon.clone();
            let player_tracker = data.player_tracker.clone();
            let channel_id = data.config.player_log_channel_id;

            rcon.lock().await.ctx = Some(ctx.clone());
            
            tokio::spawn(async move {
                check_players(
                    rcon,
                    player_tracker,
                    &ChannelId::new(channel_id),
                    cache,
                    http,
                    ctx
                )
                .await
            });
        }
        _ => {}
    }
    Ok(())
}
