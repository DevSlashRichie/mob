use chrono::{DateTime, Utc, NaiveDateTime};
use serenity::{framework::standard::{macros::{group, command}, CommandResult}, prelude::{Context}, model::prelude::{Message, UserId}};

use crate::{utils::{self}, repo, actions};

const DANIEL_PROXD: &UserId = &UserId(452644418995093526);
const DEV_SLASH_RICHIE: &UserId = &UserId(894381937651765279);

#[group]
#[commands(ping, add, list)]
pub struct General;

async fn check_owner(ctx: &Context, msg: &Message) -> bool {
    if msg.author.id != *DANIEL_PROXD && msg.author.id != *DEV_SLASH_RICHIE {
        _ = msg.reply(ctx, "Grosero tu no puedes.").await;
        false
    } else {
        true
    }
}

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
pub async fn add(ctx: &Context, msg: &Message) -> CommandResult {
    if !check_owner(ctx, msg).await {
        return Ok(());
    }

    let query = utils::find_args(&msg).join(" ");

    actions::handle_create(ctx, &query, msg.channel_id).await;

    CommandResult::Ok(())
}

#[command]
pub async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    if !check_owner(ctx, msg).await {
        return Ok(());
    }

    let args = utils::find_args(&msg);

    let page = args
        .first()
        .map(|s| s.parse::<usize>())
        .unwrap_or(Ok(1));

    if page.is_err() {
        _ = msg.reply(ctx, "La pagina debe ser un numero.").await;
        return Ok(())
    }

    let page = page.unwrap();


    let data = ctx.data.read().await; 
    let repo = data.get::<repo::MovieRepo>().unwrap();

    let movies = repo.load_all_movies().await?;

    if movies.is_empty() {
        _ = msg.reply(ctx, "No haz guardado ninguna pelicula.").await;
        return Ok(());
    }

    const PAGE_SIZE: usize = 5;

    let total_pages = (movies.len() as f64 / PAGE_SIZE as f64).ceil() as usize;

    let movies_names = movies
        .into_iter()
        .skip((page - 1) * PAGE_SIZE)
        .take(PAGE_SIZE)
        .map(|m| {
            let date = m.date.to_chrono();
            let cdmx_date = date.with_timezone(&chrono_tz::America::Mexico_City);
            let date = cdmx_date.format("%d/%m/%Y").to_string();

            format!("**{}**\nRate of {}/10\nSeen at {}\n", m.movie.title, m.rating.value(), date)
        })
        .collect::<Vec<_>>()
        .join("\n");

    msg.reply(ctx, format!("Page ({}/{})\n\n{}", page, total_pages, movies_names)).await?;

    Ok(())
}

