use crate::{constants::*, contexts::*, errors::*};
use anchor_lang::prelude::*;

pub fn add_like(ctx: Context<UpdateBlog>, _topic_id: String, _blog_id: String) -> Result<()> {
    let blog_account = &mut ctx.accounts.blog_account;

    blog_account.likes = blog_account.likes.checked_add(1).unwrap();

    Ok(())
}
