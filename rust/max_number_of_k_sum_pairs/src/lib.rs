// https://leetcode.com/problems/max-number-of-k-sum-pairs

use std::{cmp::min, collections::{HashMap, HashSet}};

pub fn max_number_of_k_sum_pairs(nums: Vec<i32>, k: i32) -> i32 {
    let mut pair_count = 0;
    let mut hm = HashMap::new();
    let mut hs = HashSet::new();
    for n in nums{
        hm.entry(n).and_modify(|e| *e += 1).or_insert(1);
    }
    for e in hm.keys() {
        let count = hm.get(e).clone().unwrap();
        let compl = k - *e;
        match hm.get(&compl) {
            Some(compl_count) if !hs.contains(e) => {
                if compl == *e {
                    pair_count += count / 2;
                } else {
                    pair_count += min(*count, *compl_count);
                    hs.insert(compl);
                }
            }
            _ => {}
        };
    }

    pair_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_number_of_k_sum_pairs_0() {
        let expected = 2;
        let actual = max_number_of_k_sum_pairs(vec![1, 2, 3, 4], 5);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_max_number_of_k_sum_pairs_1() {
        let expected = 1;
        let actual = max_number_of_k_sum_pairs(vec![3, 1, 3, 4, 3], 6);
        assert_eq!(expected, actual);
    }
}
