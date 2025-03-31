use anchor_lang::prelude::*;

#[error_code]
pub enum TopicAccountError {
    #[msg("Topic generator name too long!")]
    GeneratorNameTooLong,
    #[msg("Topic id too long!")]
    TopicIdTooLong,
    #[msg("Topic title too long!")]
    TopicTitleTooLong,
    #[msg("Topic Description too long!")]
    TopicDescriptionTooLong,
}

#[error_code]
pub enum BlogAccountError {
    #[msg("Blog generator name is too long!")]
    BlogGeneratorNameTooLong,
    #[msg("Topic id too long!")]
    TopicIdTooLong,
    #[msg("Blog id too long!")]
    BlogIdTooLong,
    #[msg("Blog too long!")]
    BlogTooLong,
}
