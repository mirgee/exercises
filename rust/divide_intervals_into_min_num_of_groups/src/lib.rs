// https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups/
use std::collections::VecDeque;

pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals = intervals.clone();
    intervals.sort_by(|v1, v2| v1[1].partial_cmp(&v2[1]).unwrap());
    let mut intervals = VecDeque::from(intervals);
    println!("Intervals: {:?}", intervals);
    let mut res = 0;
    while let Some(int) = intervals.pop_front() {
        println!("new group starting: {:?}", int);
        res += 1;
        let mut curr = int;
        let mut i = 1;
        while i < intervals.len() {
            if intervals[i][0] > curr[1] {
                curr = intervals[i].clone();
                println!("popping: {:?}", curr);
                intervals.remove(i);
                i -= 1
            }
            i += 1;
        }
    }
    res
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
