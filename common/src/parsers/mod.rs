use crate::errors::ParseError;
use std::str::FromStr;

type Result<T> = std::result::Result<T, ParseError>;

pub fn parse_pair<N>(text: &str, separator: char) -> Result<(N, N)>
where
    N: FromStr,
{
    match text.find(separator) {
        None => Err(ParseError::create(&format!(
            "Unable to find [separator={}] in [text={}]",
            separator, text
        ))),
        Some(index) => match (N::from_str(&text[..index]), N::from_str(&text[index + 1..])) {
            (Ok(l), Ok(r)) => Ok((l, r)),
            _ => Err(ParseError::create(&format!(
                "Failed to parse target type from [string={}]",
                text
            ))),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair::<i32>("10,10", ','), Ok((10, 10)));
    }
}
