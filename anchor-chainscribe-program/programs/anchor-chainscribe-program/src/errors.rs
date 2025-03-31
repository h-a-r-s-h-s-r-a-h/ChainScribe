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

