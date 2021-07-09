//! Custom errors
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StatsError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Error(&str),
    #[error("Number conversion error")]
    Conversion(#[from] std::num::TryFromIntError),
}
