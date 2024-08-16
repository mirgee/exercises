// https://leetcode.com/problems/max-consecutive-ones-iii

pub fn max_consecutive_ones_3(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut num_zeros = if nums[0] == 0 { 1 } else { 0 };
    let mut l = 0;

    while right < nums.len() {
        println!("{:?} {:?}", left, right);
        if num_zeros > k {
            if nums[left] == 0 {
                num_zeros -= 1;
            }
            left += 1;
            if right < left {
                right += 1;
                if right < nums.len() && nums[right] == 0 {
                    num_zeros += 1;
                }
            }
        } else {
            right += 1;
            if right < nums.len() && nums[right] == 0 {
                num_zeros += 1;
            }
        }
        l = std::cmp::max(l, right - left);
    }

    return l as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_consecurive_ones_3_0() {
        let expected = 6;
        let actual = max_consecutive_ones_3(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_max_consecurive_ones_3_1() {
        let expected = 10;
        let actual = max_consecutive_ones_3(
            vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
            3,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_max_consecurive_ones_3_2() {
        let expected = 3;
        let actual = max_consecutive_ones_3(vec![0, 0, 1, 1, 1, 0, 0], 0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_max_consecurive_ones_3_3() {
        let expected = 4;
        let actual = max_consecutive_ones_3(vec![0, 0, 0, 1], 4);
        assert_eq!(expected, actual);
    }
}
