// https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups/
pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_intervals_into_min_num_of_groups_0() {
        let expected = 3;
        let arg = vec![vec![5,10],vec![6,8],vec![1,5],vec![2,3],vec![1,10]];
        let actual = min_groups(arg);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_divide_intervals_into_min_num_of_groups_1() {
        let expected = 1;
        let arg = vec![vec![1,3],vec![5,6],vec![8,10],vec![11,13]];
        let actual = min_groups(arg);
        assert_eq!(expected, actual);
    }
}
