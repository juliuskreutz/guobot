use anyhow::Result;
use serenity::{
    all::{CommandInteraction, CommandOptionType},
    builder::{
        CreateCommand, CreateCommandOption, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    client::Context,
    model::Permissions,
};
use sqlx::SqlitePool;

use crate::database;

pub const NAME: &str = "add";

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

    let url = command
        .data
        .options
        .iter()
        .find(|o| o.name == "url")
        .and_then(|o| o.value.as_str())
        .unwrap();

    let message = command
        .data
        .options
        .iter()
        .find(|o| o.name == "message")
        .and_then(|o| o.value.as_str());

    let id = name.to_lowercase();
    let name = name.to_string();
    let url = url.to_string();
    let message = message.map(|s| s.to_string());

    let entry = database::DbEntry {
        id,
        name,
        url,
        message,
    };

    database::set_entry(&entry, pool).await?;

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

pub fn register() -> CreateCommand {
    CreateCommand::new(NAME)
        .description("Add entry")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "name", "Name").required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "url", "Url").required(true),
        )
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "message",
            "Message",
        ))
        .dm_permission(false)
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
