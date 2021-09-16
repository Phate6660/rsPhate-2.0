// The name is snake case. Deal with it.
#![allow(non_snake_case)]

use failure::ResultExt;
use mpris::PlayerFinder;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::http::AttachmentType;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};
use std::path::Path;

use std::env;

#[group]
#[commands(ping, wipltrn)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("^")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

async fn meta() -> (String, String, String, String, String) {
    let player_finder = PlayerFinder::new().context("Could not connect to D-Bus").unwrap();
    let player = player_finder.find_active().context("Could not find any player").unwrap();
    let player_name = player.identity();
    let metadata = player.get_metadata().context("Could not get metadata").unwrap();
    let artist = &metadata.artists().unwrap()[0];
    let album = &metadata.album_name().unwrap();
    let title = &metadata.title().unwrap();
    let cover = &metadata.art_url().unwrap();
    if player_name == "Rhythmbox" {
        let full_cover = [&cover[7..], ".jpg"].concat();
        (player_name.to_string(), 
         artist.to_string(), 
         album.to_string(), 
         title.to_string(), 
         full_cover.to_string())
    } else {
        (player_name.to_string(), 
         artist.to_string(), 
         album.to_string(), 
         title.to_string(), 
         cover.to_string())
    }
}

#[command]
async fn wipltrn(ctx: &Context, msg: &Message) -> CommandResult {
    let (player_name, artist, album, title, full_cover) = meta().await;
    let cover_with_attach = if player_name == "Rhythmbox" {
        let base_cover_vec = full_cover.split('/').collect::<Vec<&str>>();
        let base_cover = base_cover_vec[7];
        ["attachment://", base_cover].concat()
    } else {
        full_cover.to_string()
    };
    let sent_message = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("What Is Phate Listening To Right Now");
            e.url("https://libre.fm/user/phate6660");
            e.field("Player", &player_name, false);
            e.field("Title", title, false);
            e.field("Album", album, false);
            e.field("Artist", artist, false);
            e.image(cover_with_attach);
            e.color(serenity::utils::Color::PURPLE);
            e
        });
        if player_name == "Rhythmbox" {
            m.add_file(AttachmentType::Path(Path::new(&full_cover)));
        } else {
            m.add_file(AttachmentType::Image(&full_cover));
        }
        m
    }).await;
    if let Err(why) = sent_message {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}
