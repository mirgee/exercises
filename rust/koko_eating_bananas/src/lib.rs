// https://leetcode.com/problems/koko-eating-bananas

fn can_eat_in_time(piles: &Vec<i32>, eating_rate: i32, h: i32) -> bool {
    let mut hours = 0 as u64;

    for pile in piles {
        hours += (pile / eating_rate) as u64;
        if pile % eating_rate > 0 {
            hours += 1;
        }
    }

    return hours <= h as u64;
}

pub fn koko_eating_bananas(piles: Vec<i32>, h: i32) -> i32 {
    let mut min_eating_rate = 1;
    let mut max_eating_rate = *piles.iter().max().unwrap();

    while min_eating_rate <= max_eating_rate {
        let eating_rate = min_eating_rate + (max_eating_rate - min_eating_rate) / 2;
        if can_eat_in_time(&piles, eating_rate, h) {
            max_eating_rate = eating_rate - 1;
        } else {
            min_eating_rate = eating_rate + 1;
        }
    }

    return min_eating_rate;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_koko_eating_bananas_0() {
        let expected = 4;
        let actual = koko_eating_bananas(vec![3, 6, 7, 11], 8);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_koko_eating_bananas_1() {
        let expected = 30;
        let actual = koko_eating_bananas(vec![30, 11, 23, 4, 20], 5);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_koko_eating_bananas_2() {
        let expected = 23;
        let actual = koko_eating_bananas(vec![30, 11, 23, 4, 20], 6);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_koko_eating_bananas_3() {
        let expected = 3;
        let actual = koko_eating_bananas(vec![805306368, 805306368, 805306368], 1000000000);
        assert_eq!(expected, actual);
    }
}
