use anyhow::Result;
use serenity::{
    all::{CommandInteraction, CommandOptionType},
    builder::{
        CreateAutocompleteResponse, CreateCommand, CreateCommandOption, CreateEmbed,
        CreateInteractionResponse, CreateInteractionResponseMessage,
    },
    client::Context,
};
use sqlx::SqlitePool;

use crate::database;

pub const NAME: &str = "guide";

pub async fn command(ctx: &Context, command: &CommandInteraction, pool: &SqlitePool) -> Result<()> {
    let name = command
        .data
        .options
        .iter()
        .find(|o| o.name == "name")
        .and_then(|o| o.value.as_str())
        .unwrap();

    let Ok(entry) = database::get_entry_by_id(&name.to_lowercase(), pool).await else {
        return Err(anyhow::anyhow!(
            "That graphic is in another castle! <:GUOBASTARE:1134626958509092964>"
        ));
    };

    let mut response = CreateInteractionResponseMessage::new().content(entry.url);

    if let Some(message) = &entry.message {
        response = response.embed(CreateEmbed::new().title("Message").description(message));
    }

    command
        .create_response(&ctx, CreateInteractionResponse::Message(response))
        .await?;

    Ok(())
}

pub async fn autocomplete(
    ctx: &Context,
    autocomplete: &CommandInteraction,
    pool: &SqlitePool,
) -> Result<()> {
    let id = autocomplete
        .data
        .options
        .iter()
        .find(|o| o.name == "name")
        .and_then(|o| o.value.as_str())
        .unwrap_or_default()
        .to_lowercase();

    let entries = database::get_entries(pool).await?;

    let mut response = CreateAutocompleteResponse::new();

    for entry in entries
        .iter()
        .filter(|e| id.is_empty() || e.id.contains(&id))
        .take(25)
    {
        response = response.add_string_choice(&entry.name, &entry.id);
    }

    autocomplete
        .create_response(&ctx, CreateInteractionResponse::Autocomplete(response))
        .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new(NAME)
        .description("Show an entry")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "name", "Name")
                .required(true)
                .set_autocomplete(true),
        )
        .dm_permission(false)
}
