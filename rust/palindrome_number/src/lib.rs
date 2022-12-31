pub fn palindrome_number(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    if x < 10 {
        return true;
    }
    let mut x_ = x;
    let mut x_new = 0;
    while x_ > 0 {
        x_new = 10 * x_new + (x_ % 10);
        x_ /= 10;
    }
    x == x_new

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_number_0() {
        let expected = true;
        let actual = palindrome_number(121);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_palindrome_number_1() {
        let expected = false;
        let actual = palindrome_number(-121);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_palindrome_number_2() {
        let expected = false;
        let actual = palindrome_number(10);
        assert_eq!(expected, actual);
    }

}
