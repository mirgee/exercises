// https://leetcode.com/problems/find-the-highest-altitude/description/

pub fn find_highest_altitude(gain: Vec<i32>) -> i32 {
    let mut altitudes = vec![0];
    let mut i = 0;

    for g in gain {
        altitudes.push(altitudes[i] + g);
        i += 1;
    }

    *altitudes.iter().max().unwrap() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_highest_altitude_0() {
        let expected = 1;
        let actual = find_highest_altitude(vec![-5, 1, 5, 0, -7]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_highest_altitude_1() {
        let expected = 0;
        let actual = find_highest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]);
        assert_eq!(expected, actual);
    }
}
