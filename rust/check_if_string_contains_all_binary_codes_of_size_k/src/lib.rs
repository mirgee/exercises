// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/description/

use std::collections::HashSet;

pub fn check_if_string_contains_all_binary_codes_of_size_k(s: String, k: i32) -> bool {
    if s.len() < k as usize {
        return false;
    }
    let mut set = HashSet::new();
    let last_window_i = s.len() - k as usize;
    let mut i = 0;
    while i <= last_window_i {
        let substr = &s[i..i+k as usize];
        set.insert(substr.to_owned());
        i += 1;
    }
    set.len() == 2_usize.pow(k as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_string_contains_all_binary_codes_of_size_k_0() {
        let expected = true;
        let actual = check_if_string_contains_all_binary_codes_of_size_k("00110110".to_string(), 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_check_if_string_contains_all_binary_codes_of_size_k_1() {
        let expected = true;
        let actual = check_if_string_contains_all_binary_codes_of_size_k("0110".to_string(), 1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_check_if_string_contains_all_binary_codes_of_size_k_2() {
        let expected = false;
        let actual = check_if_string_contains_all_binary_codes_of_size_k("0110".to_string(), 2);
        assert_eq!(expected, actual);
    }
}
