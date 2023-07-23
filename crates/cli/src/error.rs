#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ChatGpt(#[from] chatgpt::err::Error),
    #[error(transparent)]
    ChatGptConfig(#[from] chatgpt::config::ModelConfigurationBuilderError),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    Tch(#[from] tch::TchError),

    #[error(transparent)]
    Ignore(#[from] ignore::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    StripPrefix(#[from] std::path::StripPrefixError),
}

pub type Result<T> = std::result::Result<T, Error>;
