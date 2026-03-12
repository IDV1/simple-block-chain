use serde::Serialize;
use thiserror::Error;

#[derive(Debug,Error,Serialize)]
pub enum Error {
    #[error("{0}")]
    SerializationError(String),
    #[error("{0}")]
    NotTheChose(String)
}