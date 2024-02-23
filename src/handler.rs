use anyhow::Result;
use serenity::{
    all::{Command, CommandInteraction, Interaction, Ready},
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::{Context, EventHandler},
    gateway::ActivityData,
};
use sqlx::SqlitePool;

use crate::commands;

pub struct Handler {
    pub pool: SqlitePool,
}

impl Handler {
    async fn application_command(&self, ctx: &Context, command: &CommandInteraction) -> Result<()> {
        match command.data.name.as_str() {
            commands::add::NAME => commands::add::command(ctx, command, &self.pool).await,
            commands::delete::NAME => commands::delete::command(ctx, command, &self.pool).await,
            commands::guide::NAME => commands::guide::command(ctx, command, &self.pool).await,
            commands::list::NAME => commands::list::command(ctx, command, &self.pool).await,
            commands::add_message::NAME => {
                commands::add_message::command(ctx, command, &self.pool).await
            }
            commands::delete_message::NAME => {
                commands::delete_message::command(ctx, command, &self.pool).await
            }
            _ => Ok(()),
        }
    }

    async fn autocomplete(&self, ctx: &Context, autocomplete: &CommandInteraction) -> Result<()> {
        match autocomplete.data.name.as_str() {
            commands::delete::NAME => {
                commands::delete::autocomplete(ctx, autocomplete, &self.pool).await
            }
            commands::guide::NAME => {
                commands::guide::autocomplete(ctx, autocomplete, &self.pool).await
            }
            commands::add_message::NAME => {
                commands::add_message::autocomplete(ctx, autocomplete, &self.pool).await
            }
            commands::delete_message::NAME => {
                commands::delete_message::autocomplete(ctx, autocomplete, &self.pool).await
            }
            _ => Ok(()),
        }
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        Command::set_global_commands(
            &ctx,
            vec![
                commands::add::register(),
                commands::delete::register(),
                commands::guide::register(),
                commands::list::register(),
                commands::add_message::register(),
                commands::delete_message::register(),
            ],
        )
        .await
        .unwrap();

        ctx.set_activity(Some(ActivityData::watching("@guobacertified")));
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                if let Err(e) = self.application_command(&ctx, &command).await {
                    command
                        .create_response(
                            &ctx,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .content(e.to_string())
                                    .ephemeral(true),
                            ),
                        )
                        .await
                        .unwrap();
                }
            }
            Interaction::Autocomplete(autocomplete) => {
                self.autocomplete(&ctx, &autocomplete).await.unwrap();
            }
            _ => unimplemented!(),
        };
    }
}
