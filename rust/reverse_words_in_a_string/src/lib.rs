pub fn reverse_words_in_a_string(s: String) -> String {
   let v: Vec<_> = s.trim().split(' ').rev().filter(|s| !s.is_empty()).collect();
   v.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words_in_a_string_0() {
        let expected = "blue is sky the".to_string();
        let actual = reverse_words_in_a_string("the sky is blue".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_reverse_words_in_a_string_1() {
        let expected = "world hello".to_string();
        let actual = reverse_words_in_a_string("   hello world  ".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_reverse_words_in_a_string_2() {
        let expected = "example good a".to_string();
        let actual = reverse_words_in_a_string("a good   example".to_string());
        assert_eq!(expected, actual);
    }
}
