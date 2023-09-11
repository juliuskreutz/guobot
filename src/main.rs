mod commands;
mod database;
mod handler;

use std::{env, str::FromStr};

use anyhow::Result;
use serenity::{prelude::GatewayIntents, Client};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

use crate::handler::Handler;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let discord_token = env::var("DISCORD_TOKEN").unwrap();
    let database_url = env::var("DATABASE_URL").unwrap();

    let options = SqliteConnectOptions::from_str(&database_url)?.create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;
    sqlx::migrate!().run(&pool).await?;

    Client::builder(&discord_token, GatewayIntents::non_privileged())
        .event_handler(Handler { pool })
        .await
        .unwrap()
        .start()
        .await?;

    Ok(())
}
