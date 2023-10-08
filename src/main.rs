use discord::model::Event;
use discord::Discord;

fn main() {
    let discord = Discord::from_bot_token(
        "MTExODk2NTc4NTM0ODUzODUwOA.GTqGkc.WyEDZrhyw6zn50Iq1w33eRb--qB07CMfQ_A7cI",
    )
    .expect("Wrong token");
    let (mut connection, _) = discord.connect().expect("connection failed");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                if message.content.contains("https://twitter.com/") {
                    let reply = message.content.replace("twitter.com", "fxtwitter.com");
                    let _ = discord.delete_message(message.channel_id, message.id);
                    let _ = discord.send_message(
                        message.channel_id,
                        &format!("{} sent {reply}", message.author.mention()),
                        "",
                        false,
                    );
                }
                if message.content.contains("https://x.com") {
                    let reply = message.content.replace("x.com", "fixupx.com");
                    let _ = discord.delete_message(message.channel_id, message.id);
                    let _ = discord.send_message(
                        message.channel_id,
                        &format!("{} sent {reply}", message.author.mention()),
                        "",
                        false,
                    );
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("connection closed {code:?}: {body}");
            }
            Err(err) => println!("Error received {err}"),
        }
    }
}
