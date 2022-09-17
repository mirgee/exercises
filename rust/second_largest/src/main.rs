// Given an alphanumeric string s, return the second largest numerical digit that appears in s, or
// -1 if it does not exist.
//
// An alphanumeric string is a string consisting of lowercase English letters and digits.

fn second_largest(s: String) -> i32 {
    let mut max: i32 = -1;
    let mut sec_max: i32 = -1;
    for c in s.chars() {
        if let Some(d) = c.to_digit(10) {
            let d = d as i32;
            if d > max {
                sec_max = max;
                max = d;
            } else if d < max && d > sec_max {
                sec_max = d;
            }
        }
    }
    sec_max as i32
}

#[test]
fn test_second_largest() {
    assert_eq!(second_largest("dfa12321afd".to_string()), 2);
    assert_eq!(second_largest("abc1111".to_string()), -1);
    assert_eq!(second_largest("sjhtz8344".to_string()), 4);
}
