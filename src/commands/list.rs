use anyhow::Result;
use serenity::{
    all::CommandInteraction,
    builder::{
        CreateCommand, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
    },
    client::Context,
};
use sqlx::SqlitePool;

use crate::database;

pub const NAME: &str = "list";

pub async fn command(ctx: &Context, command: &CommandInteraction, pool: &SqlitePool) -> Result<()> {
    let entries = database::get_entries(pool).await?;

    let mut names = Vec::new();
    let mut messages = Vec::new();

    for entry in entries {
        names.push(entry.name);
        messages.push(entry.message.unwrap_or_else(|| "-".to_string()));
    }

    command
        .create_response(
            &ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(
                        CreateEmbed::new()
                            .title("Entries")
                            .field("Name", names.join("\n"), true)
                            .field("Message", messages.join("\n"), true),
                    )
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new(NAME).description("List entries")
}
