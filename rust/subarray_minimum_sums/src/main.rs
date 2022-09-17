//Given an array of integers arr, find the sum of min(b), where b ranges over every (contiguous)
//subarray of arr. Since the answer may be large, return the answer modulo 10^9 + 7.

fn subarray_minimum_sums(arr: Vec<i32>) -> i32 {
    let mut stack: Vec<i32> = Vec::with_capacity(arr.len());
    let mut dp: Vec<i32> = vec![0; arr.len() + 1];
    let mut sum = 0;
    let m: i32 = 1e9 as i32 + 7;
    stack.push(-1);
    for i in 0..arr.len() {
        while *stack.last().unwrap() != -1 as i32 && arr[*stack.last().unwrap() as usize] >= arr[i] {
            stack.pop();
        }
        dp[i + 1] = dp[(*stack.last().unwrap_or(&-1) + 1) as usize] + arr[i] * (i as i32 - stack.last().unwrap_or(&-1));
        stack.push(i as i32);
        sum += dp[i + 1];
        sum %= m;
    }
    sum
}

#[test]
fn test_subarray_minimum_sums() {
    assert_eq!(subarray_minimum_sums(vec![3,1,2,4]), 17);
    assert_eq!(subarray_minimum_sums(vec![11,81,94,43,3]), 444);
}
