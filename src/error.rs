pub use thiserror::Error;

#[derive(thiserror::Error, Debug)]
pub enum IVError {
    #[error("Static error: {0}")]
    Static(&'static str),

    #[error(transparent)]
    Glob(#[from] glob::PatternError),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    PixelsError(#[from] pixels::Error),
}
