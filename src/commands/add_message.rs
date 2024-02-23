use anyhow::Result;
use serenity::{
    all::{CommandInteraction, CommandOptionType},
    builder::{
        CreateAutocompleteResponse, CreateCommand, CreateCommandOption, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    client::Context,
    model::Permissions,
};
use sqlx::SqlitePool;

use crate::database;

pub const NAME: &str = "addmessage";

pub async fn command(ctx: &Context, command: &CommandInteraction, pool: &SqlitePool) -> Result<()> {
    if !command
        .guild_id
        .map(|id| id.get() == 1118115787320868864)
        .unwrap_or_default()
    {
        return Err(anyhow::anyhow!("This has to be used in guobas server!"));
    }

    let name = command
        .data
        .options
        .iter()
        .find(|o| o.name == "name")
        .and_then(|o| o.value.as_str())
        .unwrap();

    let message = command
        .data
        .options
        .iter()
        .find(|o| o.name == "message")
        .and_then(|o| o.value.as_str())
        .unwrap();

    let id = name.to_lowercase();
    let message = message.trim();

    database::set_entry_message_by_id(&id, message, pool).await?;

    command
        .create_response(
            &ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Success!")
                    .ephemeral(true),
            ),
        )
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
        .description("Add a message to an entry")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "name", "Name")
                .required(true)
                .set_autocomplete(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "message", "Message")
                .required(true),
        )
        .dm_permission(false)
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
