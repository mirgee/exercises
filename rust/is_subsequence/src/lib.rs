pub fn is_subsequence(s: String, t: String) -> bool {
    let mut i = 0;
    let mut j = 0;

    let sc: Vec<_> = s.chars().collect();
    let tc: Vec<_> = t.chars().collect();

    while i < sc.len() && j < tc.len() {
        if sc[i] == tc[j] {
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }

    return i == sc.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subsequence_0() {
        let expected = true;
        let actual = is_subsequence("abc".to_string(), "ahbgdc".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_subsequence_1() {
        let expected = false;
        let actual = is_subsequence("axc".to_string(), "ahbgdc".to_string());
        assert_eq!(expected, actual);
    }
}
