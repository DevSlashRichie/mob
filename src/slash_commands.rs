use serenity::{prelude::Context, model::prelude::{GuildId, interaction::application_command::{CommandDataOption, CommandDataOptionValue}, command::CommandOptionType}, builder::CreateApplicationCommand};

use crate::repo;

pub async fn run_list_command(ctx: &Context, options: &[CommandDataOption]) -> Result<String, String> {
    let page = options
        .get(0)
        .map(|f_option| {
            if let Some(CommandDataOptionValue::Integer(i)) = f_option.resolved {
                if i < 1 {
                    Err("Page number must be greater than 0".to_owned())
                } else {
                    Ok(i as usize)
                }
            } else {
                Ok(1)
            }
        })
        .unwrap_or(Ok(1))?;

    let data = ctx.data.read().await;
    let repo = data.get::<repo::MovieRepo>().unwrap();

    let movies = repo.load_all_movies().await
        .map_err(|e| format!("Error loading movies: {}", e))?;

    if movies.is_empty() {
        return Ok("No movies found".to_owned());
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

    Ok(format!("Page ({}/{})\n\n{}", page, total_pages, movies_names))
}

fn register_list_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("list")
        .description("List movies in the database")
        .create_option(|option| {
            option
                .name("page")
                .description("Page number")
                .kind(CommandOptionType::Integer)
                .required(false)
        })
}


pub async fn register_commands(ctx: &Context, guild: &GuildId) {
    _ = guild.set_application_commands(&ctx,|commands| {
        commands    
        .create_application_command(|command| register_list_command(command))
    })
    .await;

}


