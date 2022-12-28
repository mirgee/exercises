// https://leetcode.com/problems/check-if-a-string-can-break-another-string/description/
pub fn check_if_can_break(s1: String, s2: String) -> bool {
    let mut s1_sorted: Vec<char> = s1.chars().collect();
    let mut s2_sorted: Vec<char> = s2.chars().collect();
    s1_sorted.sort();
    s2_sorted.sort();
    let mut all_c1_gt_c2 = true;
    let mut all_c2_gt_c1 = true;
    for (c1, c2) in s1_sorted.iter().zip(s2_sorted.iter()) {
        if c1.lt(c2) { all_c1_gt_c2 = false; }
        if c2.lt(c1) { all_c2_gt_c1 = false; }
        if !all_c1_gt_c2 && !all_c2_gt_c1 { return false; }
    }
    all_c1_gt_c2 || all_c2_gt_c1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_can_break_0() {
        let expected = true;
        let actual = check_if_can_break("abc".into(), "xya".into());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_check_if_can_break_1() {
        let expected = false;
        let actual = check_if_can_break("abe".into(), "acd".into());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_check_if_can_break_2() {
        let expected = true;
        let actual = check_if_can_break("szy".into(), "cid".into());
        assert_eq!(expected, actual);
    }
}
