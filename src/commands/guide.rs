use anyhow::Result;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::{
            application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
            autocomplete::AutocompleteInteraction,
            InteractionResponseType,
        },
    },
    prelude::Context,
};
use sqlx::SqlitePool;

use crate::database;

pub const NAME: &str = "guide";

pub async fn command(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    pool: &SqlitePool,
) -> Result<()> {
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

    let Ok(entry) = database::get_entry_by_id(&name.to_lowercase(), pool).await else {
        return Err(anyhow::anyhow!(
            "That graphic is in another castle! <:GUOBASTARE:1134626958509092964>"
        ));
    };

    command
        .create_interaction_response(&ctx, |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|d| d.content(entry.url))
        })
        .await
        .unwrap();

    Ok(())
}

pub async fn autocomplete(
    ctx: &Context,
    autocomplete: &AutocompleteInteraction,
    pool: &SqlitePool,
) -> Result<()> {
    let id = autocomplete
        .data
        .options
        .iter()
        .find(|o| o.name == "name")
        .and_then(|o| o.resolved.as_ref())
        .and_then(|o| {
            if let CommandDataOptionValue::String(s) = o {
                Some(s.clone())
            } else {
                None
            }
        })
        .unwrap_or_default()
        .to_lowercase();

    let entries = database::get_entries(pool).await?;

    autocomplete
        .create_autocomplete_response(ctx, |r| {
            for entry in entries
                .iter()
                .filter(|e| id.is_empty() || e.id.contains(&id))
                .take(25)
            {
                r.add_string_choice(&entry.name, &entry.id);
            }

            r
        })
        .await?;

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(NAME)
        .description("Show an entry")
        .create_option(|o| {
            o.name("name")
                .description("Name")
                .kind(CommandOptionType::String)
                .required(true)
                .set_autocomplete(true)
        })
        .dm_permission(false)
}
