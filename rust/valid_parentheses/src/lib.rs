use std::collections::HashMap;

pub fn valid_parentheses(s: String) -> bool {
    if s.len() % 2 == 1 {
        return false;
    }
    let cl_of = HashMap::<char, char>::from_iter(vec![
        ('{', '}'),
        ('[', ']'),
        ('(', ')'),
    ]);
    let mut stack = vec![];
    for c in s.chars() {
        if let Some(closing) = cl_of.get(&c) {
            stack.push(closing); 
        } else if stack.len() > 0 && *stack.pop().unwrap() == c {
            continue;
        } else {
            return false;
        }
    }
    stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parentheses_0() {
        let expected = true;
        let actual = valid_parentheses("()".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_valid_parentheses_1() {
        let expected = true;
        let actual = valid_parentheses("()[]{}".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_valid_parentheses_2() {
        let expected = false;
        let actual = valid_parentheses("(}".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_valid_parentheses_3() {
        let expected = false;
        let actual = valid_parentheses("]".to_string());
        assert_eq!(expected, actual);
    }
}
