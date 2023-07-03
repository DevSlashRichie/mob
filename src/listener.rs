use serenity::{prelude::{EventHandler, Context}, async_trait, model::prelude::interaction::Interaction};

use crate::{actions::{handle_interaction}, slash_commands};

use serenity::model::application::interaction::InteractionResponseType;

pub struct Handler;


#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {

        if let Interaction::MessageComponent(mut interaction) = interaction {
            handle_interaction(&ctx, &mut interaction).await;

        _ = interaction
            .create_interaction_response(&ctx.http, |r| {
                r.kind(InteractionResponseType::DeferredUpdateMessage)
            }).await;
        } else if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "list" => slash_commands::run_list_command(&ctx, &command.data.options).await,
                _ => Ok("Unknown command".to_owned()),
            };

            let real_message = match content {
                Ok(msg) => msg,
                Err(err) => format!("Error: {}", err),
            };

            _ = command.create_interaction_response(&ctx.http, |res| {
                res.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                    .interaction_response_data(|msg| msg.content(real_message))
            })
        }

    }
}
