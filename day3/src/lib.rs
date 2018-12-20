use common::errors::ParseError;
use std::result::Result;

#[derive(Debug, Clone)]
pub struct ElfClaim {
    pub id: u32,
    pub position: (usize, usize),
    pub dimensions: (usize, usize),
}

impl ElfClaim {
    pub fn parse(text: &str) -> Result<ElfClaim, ParseError> {
        Err(ParseError::create("Dummy error."))
    }
}
