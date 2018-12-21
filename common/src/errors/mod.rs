use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        write!(f, "Input error: {}", self.message)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::convert::From<std::num::ParseIntError> for ParseError {
    fn from(error: std::num::ParseIntError) -> Self {
        ParseError {
            message: format!("Failed to parse integer: {}", error.description()),
        }
    }
}

impl std::convert::From<&str> for ParseError {
    fn from(error: &str) -> Self {
        ParseError {
            message: error.to_owned(),
        }
    }
}

impl std::convert::From<String> for ParseError {
    fn from(error: String) -> Self {
        ParseError { message: error }
    }
}

impl ParseError {
    pub fn create(message: &str) -> ParseError {
        ParseError {
            message: message.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputError {
    pub message: String,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        write!(f, "Input error: {}", self.message)
    }
}

impl Error for InputError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::convert::From<std::io::Error> for InputError {
    fn from(error: std::io::Error) -> Self {
        InputError {
            message: format!("IO error: {}", error.description()),
        }
    }
}

impl std::convert::From<std::num::ParseIntError> for InputError {
    fn from(error: std::num::ParseIntError) -> Self {
        InputError {
            message: format!("Failed to parse integer: {}", error.description()),
        }
    }
}
