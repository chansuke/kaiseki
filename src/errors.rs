//! Custom errors
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StatsError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Error(String),
    #[error("Number conversion error")]
    Conversion(#[from] std::num::TryFromIntError),
}
