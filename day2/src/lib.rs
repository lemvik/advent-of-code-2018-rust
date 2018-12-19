extern crate common;

use std::collections::HashMap;

fn applicable_for_checksum(word: &str) -> (bool, bool) {
    let mut letters = HashMap::new();

    for ch in word.chars() {
        let entry = letters.entry(ch).or_insert(0);
        *entry += 1;
    }

    let mut has_two = false;
    let mut has_three = false;

    for cnt in letters.values() {
        has_two = *cnt == 2 || has_two;
        has_three = *cnt == 3 || has_three;
    }

    (has_two, has_three)
}

pub fn checksum(markers: &Vec<String>) -> u64 {
    let mut two_letters = 0;
    let mut three_letters = 0;

    for word in markers {
        match applicable_for_checksum(word) {
            (true, true) => {
                two_letters += 1;
                three_letters += 1;
            }
            (true, false) => {
                two_letters += 1;
            }
            (false, true) => {
                three_letters += 1;
            }
            _ => {}
        }
    }

    two_letters * three_letters
}

fn similar_id(first: &str, second: &str) -> Option<String> {
    let pairs = first.chars().zip(second.chars());
    let mut mismatch = -1;
    for (index, (fc, sc)) in pairs.enumerate() {
        if fc != sc {
            if mismatch != -1 {
                return None;
            }
            mismatch = index as isize;
        }
    }

    let result = first
        .chars()
        .take(mismatch as usize)
        .chain(first.chars().skip((mismatch + 1) as usize))
        .collect();
    Some(result)
}

pub fn locate_similar_packages(markers: &Vec<String>) -> Option<String> {
    for i in 0..markers.len() {
        for j in i + 1..markers.len() {
            match similar_id(&markers[i], &markers[j]) {
                v @ Some(_) => return v,
                None => {}
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_applicability() {
        assert_eq!(applicable_for_checksum("abcdef"), (false, false));
        assert_eq!(applicable_for_checksum("bababc"), (true, true));
        assert_eq!(applicable_for_checksum("abbcde"), (true, false));
        assert_eq!(applicable_for_checksum("abcccd"), (false, true));
        assert_eq!(applicable_for_checksum("aabcdd"), (true, false));
        assert_eq!(applicable_for_checksum("abcdee"), (true, false));
        assert_eq!(applicable_for_checksum("ababab"), (false, true));
    }

    #[test]
    fn test_similar_ids() {
        assert_eq!(similar_id("fghij", "fguij"), Some("fgij".to_owned()));
        assert_eq!(similar_id("abcde", "axcye"), None);
        assert_eq!(similar_id("abcde", "fghij"), None);
    }
}
