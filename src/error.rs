use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("MySQL error: {0}")]
    MySQL(#[from] mysql::Error),

    #[error("MongoDB error: {0}")]
    Mongo(#[from] mongodb::error::Error),

    #[error("MongoDB ser error: {0}")]
    MongoSer(#[from] mongodb::bson::ser::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Unrecoverable error on {0}")]
    Fatal(&'static str),
}

pub type Result<T> = std::result::Result<T, AppError>;
