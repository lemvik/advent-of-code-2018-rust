use std::error::Error;
use std::fmt;

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

pub type Result<T> = std::result::Result<T, InputError>;

pub fn read_numbers<N, R>(reader: R) -> Result<Vec<N>>
where
    R: std::io::BufRead,
    N: std::str::FromStr,
    N::Err: std::error::Error,
{
    reader
        .lines()
        .map(|s| match s {
            Ok(st) => st.parse::<N>().map_err(|err| InputError {
                message: String::from(err.description()),
            }),
            Err(err) => Err(InputError::from(err)),
        })
        .collect()
}
