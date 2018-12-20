use std::error::Error;

pub mod errors;
pub mod parsers;

pub type Result<T> = std::result::Result<T, errors::InputError>;

pub fn read_numbers<N, R>(reader: R) -> Result<Vec<N>>
where
    R: std::io::BufRead,
    N: std::str::FromStr,
    N::Err: std::error::Error,
{
    reader
        .lines()
        .map(|s| match s {
            Ok(st) => st.parse::<N>().map_err(|err| errors::InputError {
                message: String::from(err.description()),
            }),
            Err(err) => Err(errors::InputError::from(err)),
        })
        .collect()
}

pub fn read_lines<R>(reader: R) -> Result<Vec<String>>
where
    R: std::io::BufRead,
{
    reader
        .lines()
        .map(|s| s.map_err(errors::InputError::from))
        .collect()
}
