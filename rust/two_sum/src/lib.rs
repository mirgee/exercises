use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    for (i, n) in nums.iter().enumerate() {
        let i = i as i32;
        if let Some(j) = hm.get(n) {
            return vec![*j, i];
        } else {
            hm.insert(target - n, i);
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_0() {
        let expected = vec![0,1];
        let actual = two_sum(vec![2,7,11,15], 9);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_two_sum_1() {
        let expected = vec![1,2]; 
        let actual = two_sum(vec![3,2,4], 6);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_two_sum_2() {
        let expected = vec![0,1]; 
        let actual = two_sum(vec![3,3], 6);
        assert_eq!(expected, actual);
    }
}
