use crate::{constants::*, contexts::comment_context::AddComment, errors::*};
use anchor_lang::prelude::*;

// pub commenter: Pubkey,
//     pub comment_id: String,
//     pub topic_id: String,
//     pub blog_id: String,
//     pub comment_text: String,
//     pub last_updated_at: i64,

pub fn add_comment(
    ctx: Context<AddComment>,
    comment_id: String,
    blog_id: String,
    topic_id: String,
    comment_text: String,
) -> Result<()> {
    require!(
        comment_text.len() <= MAX_COMMENT_LENGTH,
        CommentAccountError::CommentTextTooLong
    );
    Ok(())
}
