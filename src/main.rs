use std::env;

use dotenvy::dotenv;
use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong").await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN not set");

    let intents = serenity::GatewayIntents::GUILDS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping()],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                println!("Connected as {}", ready.user.name);

                // 👇 DEV ONLY: register slash commands instantly in ONE guild
                let guild_id = serenity::GuildId::new(1148399716288168006);
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    guild_id,
                )
                .await?;

                Ok(())
            })
        })
        .build();

    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .await?;

    client.start().await?;
    Ok(())
}
