use std::str::FromStr;

pub fn split_pair(text: &str, separator: char) -> Option<(&str, &str)> {
    match text.find(separator) {
        None => None,
        Some(index) => Some((&text[..index], &text[index + 1..])),
    }
}

pub fn parse_pair<N>(text: &str, separator: char) -> Option<(N, N)>
where
    N: FromStr,
{
    match split_pair(text, separator) {
        None => None,
        Some((left, right)) => match (N::from_str(left), N::from_str(right)) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair::<i32>("10,10", ','), Some((10, 10)));
        assert_eq!(parse_pair::<i32>("10,", ','), None);
        assert_eq!(parse_pair::<i32>(",10", ','), None);
        assert_eq!(parse_pair::<i32>(",", ','), None);
        assert_eq!(parse_pair::<i32>("", ','), None);
        assert_eq!(parse_pair::<i32>("10,10,10", ','), None);
    }

    #[test]
    fn test_split_pair() {
        assert_eq!(split_pair("10,10", ','), Some(("10", "10")));
        assert_eq!(split_pair("10,", ','), Some(("10", "")));
        assert_eq!(split_pair(",10", ','), Some(("", "10")));
        assert_eq!(split_pair(",", ','), Some(("", "")));
        assert_eq!(split_pair("", ','), None);
        assert_eq!(split_pair("10,10,10", ','), Some(("10", "10,10")));
    }
}
