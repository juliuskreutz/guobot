use anyhow::Result;
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{
            command::CommandOptionType,
            interaction::{
                application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
                InteractionResponseType,
            },
        },
        Permissions,
    },
    prelude::Context,
};
use sqlx::SqlitePool;

use crate::database;

pub const NAME: &str = "add";

pub async fn command(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    pool: &SqlitePool,
) -> Result<()> {
    if !command
        .guild_id
        .map(|id| id.0 == 1118115787320868864)
        .unwrap_or_default()
    {
        return Err(anyhow::anyhow!("This has to be used in guobas server!"));
    }

    let name = command
        .data
        .options
        .iter()
        .find(|o| o.name == "name")
        .and_then(|o| o.resolved.as_ref())
        .and_then(|o| {
            if let CommandDataOptionValue::String(s) = o {
                Some(s)
            } else {
                None
            }
        })
        .unwrap();

    let url = command
        .data
        .options
        .iter()
        .find(|o| o.name == "url")
        .and_then(|o| o.resolved.as_ref())
        .and_then(|o| {
            if let CommandDataOptionValue::String(s) = o {
                Some(s)
            } else {
                None
            }
        })
        .unwrap();

    {
        let id = name.clone().to_lowercase();
        let name = name.clone();
        let url = url.clone();

        let entry = database::DbEntry { id, name, url };

        database::set_entry(&entry, pool).await?;
    }

    command
        .create_interaction_response(&ctx, |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|d| {
                    d.content(format!("Successfully added {name} with {url}"))
                        .ephemeral(true)
                })
        })
        .await
        .unwrap();

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(NAME)
        .description("Add entry")
        .create_option(|o| {
            o.name("name")
                .description("Name")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|o| {
            o.name("url")
                .description("Url")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .dm_permission(false)
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
