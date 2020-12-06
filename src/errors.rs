use thiserror::Error;

#[derive(Error, Debug)]
pub enum PromptError {
    #[error("io error")]
    IOError(#[from] std::io::Error),

    #[error("prompt cancelled")]
    CancelError,
}
