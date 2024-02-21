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

    // Doesn't use manager's cmd to avoid reconnect loop when using /quit
    // Allows commands to fail
    let response = rcon.connection.cmd(&cmd).await;
    if let Ok(response) = response {
        ctx.say(format!("Command response: {}", response)).await?;
        println!("Command responded {}", response);
    }

    Ok(())
}
