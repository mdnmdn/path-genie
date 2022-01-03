use std::{error::Error, fmt};

#[derive(Debug)]
pub struct AppError {
    details: String,
}

impl AppError {
    pub fn new(message: String) -> Self {
        AppError { details: message }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        &self.details
    }
}
