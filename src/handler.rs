use anyhow::Result;
use serenity::{
    async_trait,
    model::{
        application::interaction::Interaction,
        gateway::Ready,
        prelude::{
            command::Command,
            interaction::{
                application_command::ApplicationCommandInteraction,
                autocomplete::AutocompleteInteraction,
            },
            Activity, InteractionResponseType,
        },
    },
    prelude::*,
};
use sqlx::SqlitePool;

use crate::commands;

pub struct Handler {
    pub pool: SqlitePool,
}

impl Handler {
    async fn application_command(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<()> {
        match command.data.name.as_str() {
            commands::add::NAME => commands::add::command(ctx, command, &self.pool).await,
            commands::delete::NAME => commands::delete::command(ctx, command, &self.pool).await,
            commands::guide::NAME => commands::guide::command(ctx, command, &self.pool).await,
            _ => Ok(()),
        }
    }

    async fn autocomplete(
        &self,
        ctx: &Context,
        autocomplete: &AutocompleteInteraction,
    ) -> Result<()> {
        match autocomplete.data.name.as_str() {
            commands::delete::NAME => {
                commands::delete::autocomplete(ctx, autocomplete, &self.pool).await
            }
            commands::guide::NAME => {
                commands::guide::autocomplete(ctx, autocomplete, &self.pool).await
            }
            _ => Ok(()),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        Command::set_global_application_commands(&ctx.http, |command| {
            command
                .create_application_command(|command| commands::add::register(command))
                .create_application_command(|command| commands::delete::register(command))
                .create_application_command(|command| commands::guide::register(command))
        })
        .await
        .unwrap();

        ctx.set_activity(Activity::watching("@guobacertified"))
            .await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                if let Err(e) = self.application_command(&ctx, &command).await {
                    command
                        .create_interaction_response(&ctx, |r| {
                            r.kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|d| d.content(e).ephemeral(true))
                        })
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
