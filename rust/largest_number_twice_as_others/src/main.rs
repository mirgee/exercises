// https://leetcode.com/problems/largest-number-at-least-twice-of-others/description/

fn largest_number_twice_as_others(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];    
    let mut j = 0;
    for (i, n) in nums.iter().enumerate() {
        if *n > max {
            max = *n;
            j = i;
        }
    }
    for n in nums {
        if 2 * n > max && n != max { return -1; }
    }
    j as i32
}

#[test]
fn test_largest_number_twice_as_others() {
    assert_eq!(largest_number_twice_as_others(vec![3,6,1,0]), 1);
    assert_eq!(largest_number_twice_as_others(vec![1,2,3,4]), -1);
}
