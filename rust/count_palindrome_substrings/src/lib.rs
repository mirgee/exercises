// https://leetcode.com/problems/palindromic-substrings/description/

pub fn count_palindrome_substrings(s: String) -> i32 {
    let mut res = 0;
    for i in 0..s.len() {
        res += verify_substr(&s, i, i);
        res += verify_substr(&s, i, i+1);
    } 
    res
}

pub fn verify_substr(s: &str, mut l: usize, mut r: usize) -> i32 {
    let mut c = 0;
    while r < s.len() && s[l..=l] == s[r..=r] {
        c += 1;
        r += 1;
        if l > 0 { l -= 1; } else { break; }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_palindrome_substrings_0() {
        let expected = 3;
        let actual = count_palindrome_substrings("abc".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_palindrome_substrings_1() {
        let expected = 6;
        let actual = count_palindrome_substrings("aaa".to_string());
        assert_eq!(expected, actual);
    }
}
