#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ChatGpt(#[from] chatgpt::err::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
