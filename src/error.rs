pub use thiserror::Error;

#[derive(thiserror::Error, Debug)]
pub enum FBIError {
    // /// For starter, to remove as code matures.
    #[error("Generic error: {0}")]
    Generic(String),
    //
    // /// For starter, to remove as code matures.
    // #[error("Static error: {0}")]
    // Static(&'static str),
    #[error(transparent)]
    Glob(#[from] glob::PatternError),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
