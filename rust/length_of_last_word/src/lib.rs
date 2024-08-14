pub fn length_of_last_word(s: String) -> i32 {
    let v = s.trim().split(" ").filter(|s| !s.is_empty()).collect::<Vec<_>>();
    let last = v.last().unwrap();
    last.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word_0() {
        let expected = 5;
        let actual = length_of_last_word("Hello World".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_length_of_last_word_1() {
        let expected = 4;
        let actual = length_of_last_word("   fly me   to   the moon  ".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_length_of_last_word_2() {
        let expected = 6;
        let actual = length_of_last_word("luffy is still joyboy".to_string());
        assert_eq!(expected, actual);
    }
}
