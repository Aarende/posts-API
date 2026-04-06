use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetError {
    #[error("Resource not found")]
    NotFound,

    #[error("Internal error")]
    Internal
}

#[derive(Debug, Error)]
pub enum PostError {
    #[error("Resource with the same name already exists")]
    Conflict,

    #[error("Internal error")]
    Internal
}

#[derive(Debug, Error)]
pub enum PatchError {
    #[error("Resource with the same name already exists")]
    Conflict,

    #[error("Resource not found")]
    NotFound,

    #[error("Internal error")]
    Internal
}
