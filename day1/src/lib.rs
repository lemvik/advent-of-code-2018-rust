extern crate common;

use std::collections::HashSet;

pub fn compute_frequency(changes: &Vec<i64>) -> i64 {
    changes.into_iter().sum()
}

pub fn compute_repetitive_frequency(changes: &Vec<i64>) -> i64 {
    let mut current_frequency = 0;
    let mut seen_frequencies = HashSet::new();
    seen_frequencies.insert(0);

    loop {
        for change in changes {
            current_frequency += change;
            if seen_frequencies.contains(&current_frequency) {
                return current_frequency;
            } else {
                seen_frequencies.insert(current_frequency);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compute_frequency() {
        assert_eq!(compute_frequency(&vec![1, 1, 1]), 3);
        assert_eq!(compute_frequency(&vec![1, 1, -2]), 0);
        assert_eq!(compute_frequency(&vec![-1, -2, -3]), -6);
    }

    #[test]
    fn test_computed_repetitive_frequency() {
        assert_eq!(compute_repetitive_frequency(&vec![1, -1]), 0);
        assert_eq!(compute_repetitive_frequency(&vec![3, 3, 4, -2, -4]), 10);
        assert_eq!(compute_repetitive_frequency(&vec![-6, 3, 8, 5, -6]), 5);
        assert_eq!(compute_repetitive_frequency(&vec![7, 7, -2, -7, -4]), 14);
    }
}
