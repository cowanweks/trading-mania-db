// errors.rs or at the top of your file
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Entity already exists")]
    EntityAlreadyExists,

    #[error("Entity not found")]
    EntityNotFound,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Invalid password")]
    InvalidPassword,
}
