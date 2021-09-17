// The name is snake case. Deal with it.
#![allow(non_snake_case)]

mod commands;
use commands::{WIPLTRN_COMMAND, WWW_COMMAND};
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::{
    channel::Message,
    gateway::Ready,
    id::UserId
};
use serenity::framework::standard::{
    Args, CommandGroup, CommandResult, HelpOptions, StandardFramework,
    help_commands,
    macros::{
        help,
        group
    }
};
use std::collections::HashSet;
use std::env;

#[group]
#[commands(wipltrn, www)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        use serenity::model::gateway::Activity;
        use serenity::model::user::OnlineStatus;

        let activity = Activity::playing("^help for help");
        let status = OnlineStatus::Online;
        ctx.set_presence(Some(activity), status).await;
    }
}

#[help]
#[individual_command_tip = "`^help` | `^help command` | `^help group`"]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("^")) // set the bot's prefix to "^"
        .help(&MY_HELP)
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
