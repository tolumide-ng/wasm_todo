use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoErrors {
    #[error("{0}")]
    MaxReached(String),
    #[error("The value of max active is lower than the current total active tasks. Either complete/stop some of your active tasks or increase this active tasks value")]
    LowerMaxValue,
}


pub type Result<T> = std::result::Result<T, TodoErrors>;