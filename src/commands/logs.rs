use std::time::Duration;

use k8s_openapi::api::core::v1::Pod;
use futures::{AsyncBufReadExt, TryStreamExt};
use kube::{Client, Api, api::{LogParams, ListParams}};

use crate::data_types::{Context, Error};

#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn logs(ctx: Context<'_>, seconds: i64) -> Result<(), Error> {
    let namespace = "project-zomboid";
    let deployment = "pz-server";

    println!("Connecting to kube api");

    let kube_client = Client::try_default().await?;
    let pods: Api<Pod> = Api::namespaced(kube_client, namespace);

    let pod = &pods.list(&ListParams::default().labels(&format!("app={deployment}"))).await?.items[0];
    let pod_name = pod.metadata.name.clone().unwrap();

    let mut stream = pods.log_stream(&pod_name, &kube::api::LogParams {
        container: None,
        follow: true,
        since_seconds: Some(seconds),
        timestamps: false,
        ..LogParams::default()
    }).await?.lines();

    println!("Reading logs");
    let mut res = "".to_string();
    let _err = tokio::time::timeout(Duration::from_secs(2), async {
        while let Ok(Some(line)) = stream.try_next().await {
            res += &(line + "\n");
        }
    }).await;

    ctx.reply(format!("Logs from last {seconds} seconds: ")).await?;
    let mut accumulator = "".to_string();
    let lines = res.split('\n');
    for line in lines {
        if line.contains("RCON") { continue; }

        if accumulator.len() + line.len() > 1990 {
            ctx.reply(accumulator).await?;
            accumulator = "".to_string();
        }
        accumulator += &format!("{line}\n");
    }
    ctx.reply(accumulator).await?;
    ctx.reply("End logs").await?;
    println!("Logs printed");

    Ok(())
}
