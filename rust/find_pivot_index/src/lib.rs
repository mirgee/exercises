// https://leetcode.com/problems/find-pivot-index

pub fn find_pivot_index(nums: Vec<i32>) -> i32 {
    let total_sum: i32 = nums.iter().sum();
    let mut left_sum = 0;

    for (i, num) in nums.iter().enumerate() {
        if left_sum * 2 == total_sum - num {
            return i as i32;
        }
        left_sum += num;
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pivot_index_0() {
        let expected = 3;
        let actual = find_pivot_index(vec![1, 7, 3, 6, 5, 6]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_pivot_index_1() {
        let expected = -1;
        let actual = find_pivot_index(vec![1, 2, 3]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_pivot_index_2() {
        let expected = 0;
        let actual = find_pivot_index(vec![2, 1, -1]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_pivot_index_3() {
        let expected = 2;
        let actual = find_pivot_index(vec![-1, -1, -1, -1, -1, 0]);
        assert_eq!(expected, actual);
    }
}
