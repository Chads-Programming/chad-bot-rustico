use serenity::all::{CreateEmbed, CreateEmbedFooter};

use crate::{errors::CustomError, router::book_club::BookSubmission};

pub async fn create_book_embeb(book: BookSubmission) -> Result<CreateEmbed, CustomError> {
    let embed = CreateEmbed::new()
        .title(book.title)
        .description(format!(
            "{}\n\nvotes: `{}`\npowered by: <@{}>",
            book.description, book.votes, book.creator_id
        ))
        .footer(CreateEmbedFooter::new(book.authors.join("-").to_string()))
        .image(book.cover_url);

    Ok(embed)
}
