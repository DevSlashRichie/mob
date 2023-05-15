use mongodb::bson::DateTime;
use serenity::{prelude::Context, builder::{CreateMessage, EditMessage, CreateEmbed}, model::prelude::{interaction::{message_component::MessageComponentInteraction, Interaction}, component::ButtonStyle, ChannelId}};
use anyhow::Result;
use crate::{utils::{self as cutils, Rating}, movie_api::MovieDb, repo::{MovieRepo, RatedMovie}};

use crate::{query_handler::{QueryMessage, QueryHandler}, tmdb::model::Movie};

fn create_embed(e: &mut CreateEmbed, movie: &Movie, page: usize, max_page: usize) {
    e
        .title(&movie.title)
        .description(&movie.overview)
        .color(cutils::random_color())
        .footer(|f| {
            f.text(format!(
                "Page: {} / {}",
                page, 
                max_page
            )
            )
        });

    if let Some(img) = &movie.poster_path {
        e.image(img);
    }

}

fn create_movie_message<'a>(msg: &mut CreateMessage, movie: &'a Movie, query: &'a str, size: usize) {
    msg
        .content(format!("Showing results for: {}", query))
        .embed(|e| {
            create_embed(e, movie, 1, size);
            e
        });
}


fn edit_movie_message(c: &mut EditMessage, movie: &Movie, query: &QueryMessage) {
    c
        .content(format!("Showing results for: {}", &query.query))
        .embed(|e| {
            create_embed(e, movie, query.current_index + 1, query.size);
            e
        });
}

pub async fn handle_create(ctx: &Context, query: &str, ch: ChannelId) {
    let data = ctx.data.read().await;

    let query_handler = data.get::<QueryHandler>().unwrap();
    let movies_repo = data.get::<MovieDb>().unwrap().as_ref();

    let movies = movies_repo.find_movies(&query).await.unwrap().results;
    let size = movies.len();

    if movies.is_empty() {
        let _ = ch.send_message(&ctx.http, |m| {
            m.content(format!("No results found for: {}", query))
        }).await;

        return;
    }

    let movie = movies.into_iter().next().unwrap();

    let msg  = ch
        .send_message(&ctx.http, |c| {
            create_movie_message(c, &movie, query, size);

            c.components(|cc| {
                if size > 1 {
                    cc.create_action_row(|r| {
                        r.create_button(|b| {
                            b
                                .label("Next")
                                .style(ButtonStyle::Primary)
                                .custom_id("adv_next_movie")
                        });

                        r
                    });
                }
                cc.create_action_row(|r| {
                    r.create_select_menu(|s| {
                        s
                            .custom_id("add_movie")
                            .options(|o| {
                                (1..=10).for_each(|i| {
                                    o.create_option(|o| {
                                        o.label(format!("Rating: {}", i))
                                            .value(i.to_string())
                                            .description(Rating::new(i).unwrap().to_stars())
                                    });
                                });
                                o
                            })
                    })
                })
            });

            c 
        })
    .await;

    match msg {
        Ok(msg) => {
            query_handler.create_query(msg.id.clone(), QueryMessage {
                id: msg.id,
                query: query.to_string(),
                current_index: 0,
                size
            })
        },

        Err(e) => {
            println!("Error: {}", e);
        }

    }

}

pub async fn handle_interaction(ctx: &Context, interaction: &mut MessageComponentInteraction) -> Result<()> {
    if interaction.data.custom_id.starts_with("adv") {
        handle_page_change(&ctx, interaction).await?;
    } else if interaction.data.custom_id == "add_movie" {
        handle_rate(&ctx, interaction).await?;
    } 

    Ok(())
}

async fn handle_rate(ctx: &Context, ch: &mut MessageComponentInteraction) -> Result<()> {
    let data = ctx.data.read().await;
    let query_handler = data.get::<QueryHandler>().unwrap();

    let current_movie = query_handler.current(&ch.message.id);

    if let Some(query) = current_movie {
        let movies_repo = data.get::<MovieDb>().unwrap().as_ref();
        let movies = movies_repo.find_movies(&query.query).await?;
        let movie = &movies.results[query.current_index];

        let rating = ch.data.values.first().unwrap().parse::<u8>()?;

        let repo = data.get::<MovieRepo>().unwrap();

        repo.create_movie(RatedMovie {
            movie: movie.clone(),
            rating: Rating::new(rating)?,
            date: DateTime::now()
        }).await?;

        _ = ch.channel_id.send_message(&ctx, |c| {
            c.content(format!("Added {} to your list", movie.title))
        }).await;

        _ = ch.message.delete(&ctx).await;
    }

    Ok(())
}

async fn handle_page_change(ctx: &Context, ch: &mut MessageComponentInteraction) -> Result<()> {
    let next = if ch.data.custom_id == "adv_next_movie" { true } else { false };

    let data = ctx.data.read().await;

    let query_handler = data.get::<QueryHandler>().unwrap();

    let msg_id = &ch.message.id;

    let query = if next {
        query_handler.next(msg_id)
    } else {
        query_handler.previous(msg_id)
    }.unwrap();

    let movies_repo = data.get::<MovieDb>().unwrap().as_ref();

    let movies = movies_repo.find_movies(&query.query).await?;
    let movie = &movies.results[query.current_index];

    _ = ch
        .message
        .edit(&ctx.http, |c| {
            edit_movie_message(c, &movie, &query);

            c.components(|cc| {
                cc
                    .create_action_row(|r| {
                        if query.current_index > 0 {
                            r.create_button(|b| {
                                b
                                    .label("Previous")
                                    .style(ButtonStyle::Primary)
                                    .custom_id("adv_prev_movie")

                            });
                        }

                        if query.has_next() {
                            r.create_button(|b| {
                                b
                                    .label("Next")
                                    .style(ButtonStyle::Primary)
                                    .custom_id("adv_next_movie")
                            });
                        }

                        r
                    })
                    .create_action_row(|r| {
                        r.create_select_menu(|s| {
                            s
                                .custom_id("add_movie")
                                .options(|o| {
                                    (1..=10).for_each(|i| {
                                        o.create_option(|o| {
                                            o.label(format!("Rating: {}", i))
                                                .value(i.to_string())
                                                .description(Rating::new(i).unwrap().to_stars())
                                        });
                                    });
                                    o
                                })
                        })
                    })
            });

            c
        })
        .await;

    Ok(())
}

