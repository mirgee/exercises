// https://leetcode.com/problems/roman-to-integer/description/
// Since we are starting from the back, we can safely add 
pub fn roman_to_integer_0(s: String) -> i32 {
    s.chars().rfold(0, |acc, c| {
        acc + match c {
            'I' if acc >= 5 => -1,
            'I' => 1,
            'V' => 5,
            'X' if acc >= 50 => -10,
            'X' => 10,
            'L' => 50,
            'C' if acc >= 500 => -100,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    })
}

pub fn roman_to_integer_1(s: String) -> i32 {
    //create dictionary to store roman values
    use std::collections::HashMap;
    let roman_lib: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    //variable to store sum
    let mut sum: i32 = 0;
    //store previous number
    let mut prev: i32 = 0;
    // convert s to chars in uppercase, reverse iterate,
    //convert from right to left
    for char in s.to_uppercase().chars().rev() {
        if let Some(num) = roman_lib.get(&char) {
            if *num < prev {
                sum -= num;
            } else {
                sum += num;
            }
            prev = *num;
        } else {
            panic!("Contains non-roman number [{}]", char)
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_integer_0() {
        let expected = 3;
        let actual = roman_to_integer_0("III".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_roman_to_integer_2() {
        let expected = 1801;
        let actual = roman_to_integer_0("MDCCCI".to_string());
        assert_eq!(expected, actual);
    }
}
