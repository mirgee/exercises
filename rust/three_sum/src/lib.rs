// https://leetcode.com/problems/3sum/description/
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    let mut nums = nums.clone();
    nums.sort();
    for left in 0..nums.len() - 1 {
        if left > 0 && nums[left] == nums[left-1] { continue; }
        let mut mid = left + 1;
        let mut right = nums.len() - 1;
        while mid < right {
            let s = nums[left] + nums[mid] + nums[right];
            if s == 0 {
                res.push(vec![nums[left], nums[mid], nums[right]]);
                while mid < right && nums[mid] == nums[mid+1] { mid += 1; }
                while mid < right && nums[right] == nums[right-1] { right -= 1; }
                mid += 1;
                right -= 1;
            } else if s < 0 { mid += 1; }
            else if s > 0 { right -= 1; }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum_0() {
        let expected = vec![vec![-1,-1,2],vec![-1,0,1]];
        let actual = three_sum(vec![-1,0,1,2,-1,-4]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_three_sum_1() {
        let expected: Vec<Vec<i32>> = Vec::new();
        let actual = three_sum(vec![0,1,1]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_three_sum_2() {
        let expected = vec![vec![0,0,0]];
        let actual = three_sum(vec![0,0,0]);
        assert_eq!(expected, actual);
    }
}
