use std::io;

pub fn compute_frequency<R>(reader: R) -> std::result::Result<i64, String>
where
    R: io::BufRead,
{
    let elements: std::result::Result<Vec<i64>, String> = reader
        .lines()
        .map(|s| match s {
            Ok(st) => st
                .parse::<i64>()
                .map_err(|err| format!("Failed to parse int: {}", err)),
            Err(_) => Err(String::from("Failed to read line.")),
        })
        .collect();

    match elements {
        Ok(ref v) => Ok(v.into_iter().sum()),
        Err(s) => Err(s),
    }
}
