use crate::{constants::*, contexts::*, errors::*};
use anchor_lang::prelude::*;

pub fn update_blog(
    ctx: Context<UpdateBlog>,
    topic_id: String,
    blog_id: String,
    blog: String,
) -> Result<()> {
    require!(blog.len() <= MAX_BLOG_SIZE, BlogAccountError::BlogTooLong);

    let blog_account = &mut ctx.accounts.blog_account;

    require!(
        blog_account.blog_generator == ctx.accounts.blog_generator.key(),
        BlogAccountError::AdminNotFound
    );

    blog_account.blog = blog;

    let topic = &mut ctx.accounts.topic;
    topic.last_updated_at = Clock::get()?.unix_timestamp;

    Ok(())
}
