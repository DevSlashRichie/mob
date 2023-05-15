use serenity::{prelude::{EventHandler, Context}, async_trait, model::prelude::interaction::Interaction};

use crate::{actions::{handle_interaction}};

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
        }

    }
}
