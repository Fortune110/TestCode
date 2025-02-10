use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Unknown service error")]
    Unknown,
}

pub type ServiceResult<T> = Result<T, ServiceError>;
