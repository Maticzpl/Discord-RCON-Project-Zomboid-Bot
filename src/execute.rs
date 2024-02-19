use crate::data_types::{Context, Error};

#[poise::command(slash_command)]
pub async fn execute(ctx: Context<'_>, cmd: String) -> Result<(), Error> {
    let u = ctx.author();
    if !u.has_role(
            ctx,
            ctx.guild_id().unwrap(),
            ctx.data().config.admin_role_id,
        ).await?
    {
        return Ok(());
    }

    let mut rcon = ctx.data().rcon.lock().await;
    println!("Sending RCON command: {}", &cmd);

    match rcon.cmd(&cmd).await {
        Ok(response) => {
            ctx.say(format!("Command response: {}", response)).await?;
        }
        Err(error) => {
            ctx.say(format!("Command error: {}", error)).await?;
        }
    }

    Ok(())
}
