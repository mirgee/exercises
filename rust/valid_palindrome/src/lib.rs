pub fn valid_palindrome(s: String) -> bool {
    let chars: Vec<_> = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect();
    let mut i = 0;
    let mut j = chars.len().saturating_sub(1);
    while i < j {
        if chars[i] != chars[j] {
            return false
        }
        i += 1;
        j = j.saturating_sub(1);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_palindrome_0() {
        let expected = true;
        let actual = valid_palindrome("A man, a plan, a canal: Panama".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_valid_palindrome_1() {
        let expected = false;
        let actual = valid_palindrome("race a car".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_valid_palindrome_2() {
        let expected = true;
        let actual = valid_palindrome(" ".to_string());
        assert_eq!(expected, actual);
    }
}
