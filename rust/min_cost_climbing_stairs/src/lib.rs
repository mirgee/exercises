// https://leetcode.com/problems/min-cost-climbing-stairs

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut dp = Vec::<i32>::with_capacity(cost.len()+1);
    dp.resize(cost.len()+1, 0);
    dp[cost.len()] = 0;
    dp[cost.len()-1] = *cost.last().unwrap();
    for step in (0..=cost.len()-2).rev() {
        dp[step] = cost[step] + std::cmp::min(dp[step + 1], dp[step + 2]);
    }
    std::cmp::min(dp[0], dp[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_climbing_stairs_0() {
        let expected = 15;
        let actual = min_cost_climbing_stairs(vec![10, 15, 20]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_min_cost_climbing_stairs_1() {
        let expected = 6;
        let actual = min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]);
        assert_eq!(expected, actual);
    }
}
