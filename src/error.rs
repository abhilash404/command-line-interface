use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("List not found: {0}")]
    ListNotFound(String),
    #[error("Item not found: {0}")]
    ItemNotFound(String),
}

pub type TodoResult<T> = Result<T, TodoError>;