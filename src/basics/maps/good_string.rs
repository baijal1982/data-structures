use std::collections::{HashMap, HashSet};

pub fn are_occurrences_equal(s: String) -> bool {
    if s.is_empty() || s.len() < 2 {
        return false;
    }

    let new_string = s.to_lowercase();
    let data = new_string.chars().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    data.into_iter()
        .map(|(_, n)| n)
        .collect::<HashSet<i32>>()
        .len()
        == 1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_occurances_equal() {
        let str = "abacbc";
        assert_eq!(true, are_occurrences_equal(str.to_string()));
    }

    #[test]
    fn test_occurances_not_equal() {
        let str = "tveixwaeoezcf";
        assert_eq!(false, are_occurrences_equal(str.to_string()));
    }

    #[test]
    fn test_empty_case() {
        let str = "";
        assert_eq!(false, are_occurrences_equal(str.to_string()));
    }
}
