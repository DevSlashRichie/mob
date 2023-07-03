mod commands;
mod repo;
mod movie_api;
mod tmdb;
mod utils;
mod listener;
mod query_handler;
pub mod actions;
mod slash_commands;

use std::env;

use listener::Handler;
use movie_api::MovieDb;
use query_handler::QueryHandler;
use repo::MovieRepo;
use serenity::{prelude::GatewayIntents, Client};
use serenity::framework::StandardFramework;
use dotenv::dotenv;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let discrd_token = env::var("DISCORD_TOKEN").expect("discord token");
    let mongo_host = env::var("MONGO_HOST").expect("mongo host");
    let api_key = env::var("MOVIE_DB_API_KEY").expect("MOVIE_DB_API_KEY");

    let repo = repo::MovieRepo::load(&mongo_host).await.expect("Error loading repo");
    let movie_db = movie_api::MovieDb::load(api_key);

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) 
        .group(&commands::GENERAL_GROUP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut client = Client::builder(discrd_token, intents)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<MovieRepo>(repo)
        .type_map_insert::<MovieDb>(movie_db)
        .type_map_insert::<QueryHandler>(QueryHandler::default())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error ocurred while running the client: {:?}", why);
    }
}
