use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Item not found: {0}")]
    ItemNotFound(String),

    #[error("List not found: {0}")]
    ListNotFound(String),
}

pub type TodoResult<T> = Result<T, TodoError>;