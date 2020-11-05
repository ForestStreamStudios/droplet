use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use std::{env, fs};
use rand::Rng;

#[group]
#[commands(ping, whoami, story_comedy, story_horror)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("fss!")) // set the bot's prefix to "~"
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

#[command]
async fn whoami(ctx: &Context, msg: &Message) -> CommandResult {
    let name: &str = &*msg.author.name;
    match name {
        "Bobbbay" => msg.reply(ctx, "The coolest kid around.").await?,
        "Gdog" => msg.reply(ctx, "The second coolest guy around.").await?,
        "Carly Rae" => msg.reply(ctx, "No.").await?,
        "Char_Latte" => msg.reply(ctx, "Spelt Charlotte*.").await?,
        "Proud_Imagination_94" => msg.reply(ctx, "The dedicated, talented and cheeky Giraffe.").await?,
        "jkrazy" => msg.reply(ctx, "Lego man (+ a cool beard).").await?,
        "Octagonal" => msg.reply(ctx, "The cute profile picture chief wiz.").await?,
        _ => msg.reply(ctx, "You ain't in the list yet.").await?
    };

    Ok(())
}

#[command]
async fn story_comedy(ctx: &Context, msg: &Message) -> CommandResult {
    let rng = rand::thread_rng().gen_range(0, 1);

    let filename = "data/stories_comedy.json";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file...");
    let stories: serde_json::Value = serde_json::from_str(&contents).expect("JSON was not well-formatted...");

    let tale = &stories[rng.to_string()];
    msg.reply(ctx, tale).await?;

    Ok(())
}

#[command]
async fn story_horror(ctx: &Context, msg: &Message) -> CommandResult {
    let rng = rand::thread_rng().gen_range(0, 8);

    let filename = "data/stories_horror.json";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file...");
    let stories: serde_json::Value = serde_json::from_str(&contents).expect("JSON was not well-formatted...");

    let tale = &stories[rng.to_string()];
    msg.reply(ctx, tale).await?;

    Ok(())
}
