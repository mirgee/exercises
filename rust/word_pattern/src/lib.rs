use std::collections::{HashMap, HashSet};

pub fn word_pattern(pattern: String, s: String) -> bool {
    let mut hm = HashMap::new();
    let words = s.split(" ").collect::<Vec<_>>();

    let mut already_seen = HashSet::new();

    if words.len() != pattern.len() {
        return false;
    }

    for (c, word) in pattern.chars().zip(words) {
        if let Some(w) = hm.insert(c, word) {
            if w != word {
                return false
            }
        } else if already_seen.contains(word) {
            return false;
        }
        already_seen.insert(word);
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_pattern_0() {
        let expected = true;
        let actual = word_pattern("abba".to_string(), "dog cat cat dog".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_word_pattern_1() {
        let expected = false;
        let actual = word_pattern("abba".to_string(), "dog cat cat fish".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_word_pattern_2() {
        let expected = false;
        let actual = word_pattern("aaaa".to_string(), "dog cat cat dog".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_word_pattern_3() {
        let expected = false;
        let actual = word_pattern("abba".to_string(), "dog dog dog dog".to_string());
        assert_eq!(expected, actual);
    }
}
