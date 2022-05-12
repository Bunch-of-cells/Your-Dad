use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Dad;

#[async_trait]
impl EventHandler for Dad {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if let Some(sentence) = msg
            .content
            .split("I am")
            .skip(1)
            .next()
            .or_else(|| msg.content.split("I'm").skip(1).next())
            .and_then(|name| name.trim().strip_suffix([';', '.', '!']))
        {
            let _ = msg
                .channel_id
                .say(&ctx.http, format!("Hello {}, I am dad", sentence))
                .await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(
        &token,
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Dad)
    .await
    .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
