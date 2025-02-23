use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct RewriteURLHandler;

#[async_trait]
impl EventHandler for RewriteURLHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        let reply;
        if msg.content.contains("https://twitter.com/") {
            reply = msg.content.replace("twitter.com", "fxtwitter.com");
        } else if msg.content.contains("https://x.com") {
            reply = msg.content.replace("x.com", "fixupx.com");
        } else if msg.content.contains("https://reddit.com") {
            reply = msg.content.replace("reddit.com", "rxddit.com");
        } else if msg.content.contains("https://tiktok.com") {
            reply = msg.content.replace("tiktok.com", "vxtiktok.com");
        } else {
            return;
        }
        if let Err(err) = msg.delete(&ctx).await {
            println!("Error deleting message: {err:?}");
        }
        if let Err(err) = msg
            .channel_id
            .say(&ctx, &format!("{} sent {reply}", msg.author.mention()))
            .await
        {
            println!("Error sending message: {err:?}");
        }
    }
}

#[tokio::main]
async fn main() {
    let token = "TOKEN";
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(RewriteURLHandler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
