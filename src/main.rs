mod data_types;
mod execute;
mod player_tracker;
mod rcon_manager;

use std::sync::Arc;

use data_types::Error;
use poise::serenity_prelude::{self as serenity};
use tokio::{fs, sync::Mutex};
use tokio_cron_scheduler::{JobScheduler, Job};

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

    let scheduler = JobScheduler::new().await.expect("Creating periodic task scheduler failed");
    println!("Connecting to RCON");
    let rcon = RCONManager::new(config.rcon.clone()).await;

    let rcon = Arc::new(Mutex::new(rcon));

    let player_tracker = Arc::new(Mutex::new(PlayerTrackingData {
        previous_player_list: vec![],
        first: true,
    }));

    for task in config.tasks.clone() {
        let task_rcon = rcon.clone();
        let name = task.name.clone();
        scheduler.add(
            Job::new_async(task.cron_time.clone(), move |_uuid, _lock| {
                let task_rcon = task_rcon.clone();
                let cmds = task.commands.clone();
                let name = task.name.clone();
                let timeout = task.timeout_minutes;
                Box::pin(async move {
                    println!("Started task {}", name);
                    let task_rcon = tokio::time::timeout(
                        tokio::time::Duration::from_mins(timeout),
                        task_rcon.lock()
                    ).await;

                    if task_rcon.is_err() {
                        println!("Task '{}' timed out", &name);
                        return
                    }
                    let mut task_rcon = task_rcon.unwrap();

                    println!("  RCON lock acquired for {}", &name);
                    for command in &cmds {
                        println!("  Running {}", command);
                        match task_rcon.connection.cmd(command).await {
                            Ok(resp) => println!("  {}", resp.replace("\n", "\n  ")),
                            Err(err) => println!("  Command failed: {}", err),
                        }
                    }
                })
            }).expect("Creating periodic task failed")
        ).await.expect("Failed to add periodic task");
        println!("Found task '{}'", &name);
    }

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

    scheduler.start().await.expect("Starting scheduler failed");

    client.unwrap().start().await.unwrap();
}



async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data
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

            rcon.lock().await.ctx = Some(ctx.clone());

            let config = data.config.clone();
            
            tokio::spawn(async move {
                check_players(
                    rcon,
                    player_tracker,
                    &config,
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
