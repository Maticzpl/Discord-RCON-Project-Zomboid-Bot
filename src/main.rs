use std::process::Command;

// use rcon_rs::{Client, PacketType};
use rcon::{Connection, AsyncStdStream};
use poise::serenity_prelude::{self as serenity};
use serde::{Serialize, Deserialize};
use tokio::{fs, sync::Mutex};

#[derive(Serialize, Deserialize)]
struct Config {
    token: String,
    admin_role_id: u64,
    rcon: ConfigRcon
}

#[derive(Serialize, Deserialize)]
struct ConfigRcon {
    address: String,
    port: String,
    password: String
}

struct Data {
    config: Config,
    rcon: Mutex<Connection<AsyncStdStream>>
} 
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn execute(
    ctx: Context<'_>,
    cmd: String
) -> Result<(), Error> {
    let u = ctx.author();
    if !u.has_role(ctx, ctx.guild_id().unwrap(), ctx.data().config.admin_role_id).await? {
        return Ok(());
    }
   

    let mut rcon = ctx.data().rcon.lock().await; 
    match rcon.cmd(&cmd).await {
        Ok(response) => {
            ctx.say(format!("Command response: {}", response)).await?;
        },
        Err(error) => {
            ctx.say(format!("Command error: {}", error)).await?;
        }
    }

    // let mut kubecli = Command::new("sh");
    // kubecli.args(["-c", "kubectl config set-context --current --namespace=project-zomboid && kubectl rollout restart deploy/pz-server"]);
    // kubecli.output()?;

    Ok(())
}


#[tokio::main]
async fn main() {
    let config: Config = serde_json::from_str(     
            &fs::read_to_string("./config.json").await
            .expect("Can't find config.json")
        ).expect("Parsing config.json failed");

    println!("Connecting to rcon");
    let rcon = <Connection<AsyncStdStream>>::builder()
        .connect(format!("{}:{}", &config.rcon.address, &config.rcon.port), &config.rcon.password)
        .await.expect("RCON Connection failed");

    let data: Data = Data { config, rcon: Mutex::new(rcon) };

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
    _data: &Data,
) -> Result<(), Error> {
    #[allow(clippy::single_match)]
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);

            // ctx.set_presence(Some(
            //     serenity::ActivityData { 
            //         name: "TEST".to_owned(), 
            //         kind: serenity::ActivityType::Playing, 
            //         state: None, 
            //         url: None 
            //     }), 
            //     serenity::OnlineStatus::Online
            // );
        },
        _ => {}
    }
    Ok(())
}
