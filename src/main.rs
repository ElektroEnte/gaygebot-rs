use std::env;
use tokio;
use crate::bot::{Bot, LogMode};

mod bot;

#[tokio::main]
async fn main() {
    // Requires two env variables bot_login containing the login name of the bot account and
    // bot_auth containing the oauth token of the bot.
    let mut bot = Bot::new(env::var("bot_login").unwrap(), env::var("bot_oauth").ok());
    bot.run(vec![bot.login.to_owned(), "mzntori".to_string()], LogMode::Debug).await.unwrap();
}
