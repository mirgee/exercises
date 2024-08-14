//https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/description/

use std::collections::HashSet;
use std::cmp::max;

fn _split_strings_into_max_unique_substrings(s: &str, ps: &mut HashSet<String>) -> i32 {
    let mut subans = 0;
    println!("ps: {:?}", ps);
    for i in 1..s.len() + 1 {
        let substr: String = s[..i].into();
        if !ps.contains(&substr) {
            ps.insert(substr.clone());
            subans = max(subans, 1 + _split_strings_into_max_unique_substrings(&s[i..], ps));
            ps.remove(&substr);
        }
    }
    subans
}

pub fn split_strings_into_max_unique_substrings(s: String) -> i32 {
    let mut ps = HashSet::new();
    _split_strings_into_max_unique_substrings(s.as_str(), &mut ps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_strings_into_max_unique_substrings_0() {
        let expected = 5;
        let actual = split_strings_into_max_unique_substrings("ababccc".into());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_split_strings_into_max_unique_substrings_1() {
        let expected = 2;
        let actual = split_strings_into_max_unique_substrings("aba".into());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_split_strings_into_max_unique_substrings_2() {
        let expected = 1;
        let actual = split_strings_into_max_unique_substrings("aa".into());
        assert_eq!(expected, actual);
    }
}
