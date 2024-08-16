use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_to_t = HashMap::<char, char>::new();
    let mut t_to_s = HashMap::<char, char>::new();

    for (sc, tc) in s.chars().zip(t.chars()) {
        let val1 = s_to_t.entry(sc).or_insert(tc);
        let val2 = t_to_s.entry(tc).or_insert(sc);

        if *val1 != tc || *val2 != sc {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic_0() {
        let expected = true;
        let actual = is_isomorphic("egg".to_string(), "add".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_isomorphic_1() {
        let expected = false;
        let actual = is_isomorphic("foo".to_string(), "bar".to_string());
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn test_is_isomorphic_2() {
        let expected = true;
        let actual = is_isomorphic("paper".to_string(), "title".to_string());
        assert_eq!(expected, actual);
    }
}
