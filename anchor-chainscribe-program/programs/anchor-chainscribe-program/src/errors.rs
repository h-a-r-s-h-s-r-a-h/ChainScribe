use anchor_lang::prelude::*;

#[error_code]
pub enum TopicAccountError {
    #[msg("Topic generator name is too large.")]
    GeneratorNameTooLarge,
    #[msg("")]

}


//     pub topic_id: String,
//     pub topic_title: String,
//     pub topic_description: String,
//     pub no_of_blog: u32,
//     pub likes: u32,
//     pub comments: u32,
//     pub is_active: bool,
//     pub is_public: bool,
//     pub last_updated_at: i64,
