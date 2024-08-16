// https://leetcode.com/problems/merge-strings-alternately

pub fn merge_strings_alternately(word1: String, word2: String) -> String {
    let mut new = String::new();
    let mut i = 0;
    let mut j = 0;

    while i < word1.len() || j < word2.len() {
        if i < word1.len() {
            new.push(word1.chars().nth(i).unwrap());
            i += 1;
        }

        if j < word2.len() {
            new.push(word2.chars().nth(j).unwrap());
            j += 1;
        }

    }

    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_strings_alternately_0() {
        let expected = "apbqcr".to_string();
        let actual = merge_strings_alternately("abc".to_string(), "pqr".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_merge_strings_alternately_1() {
        let expected = "apbqrs".to_string();
        let actual = merge_strings_alternately("ab".to_string(), "pqrs".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_merge_strings_alternately_3() {
        let expected = "apbqcd".to_string();
        let actual = merge_strings_alternately("abcd".to_string(), "pq".to_string());
        assert_eq!(expected, actual);
    }
}
