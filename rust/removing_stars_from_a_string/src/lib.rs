// https://leetcode.com/problems/removing-stars-from-a-string

pub fn removing_stars_from_a_string(s: String) -> String {
    let mut stack = vec![];
    for c in s.chars() {
        if c == '*' {
            stack.pop();
        } else {
            stack.push(c);
        }
    }

    stack.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_removing_stars_from_a_string_0() {
        let expected = "lecoe".to_string();
        let actual = removing_stars_from_a_string("leet**cod*e".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_removing_stars_from_a_string_1() {
        let expected = "";
        let actual = removing_stars_from_a_string("erase*****".to_string());
        assert_eq!(expected, actual);
    }
}
