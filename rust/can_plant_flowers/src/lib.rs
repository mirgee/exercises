// https://leetcode.com/problems/can-place-flowers/

pub fn can_plant_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
    let mut max = 0;
    for i in 0..flowerbed.len() {
        let prev = if i == 0 { 0 } else { flowerbed[i - 1] };
        let next = if i == flowerbed.len() - 1 {
            0
        } else {
            flowerbed[i + 1]
        };
        if prev == 0 && next == 0 && flowerbed[i] == 0 {
            flowerbed[i] = 1;
            max += 1;
        }
    }
    max >= n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_plant_flowers_0() {
        let expected = true;
        let actual = can_plant_flowers(vec![1, 0, 0, 0, 1], 1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_can_plant_flowers_1() {
        let expected = false;
        let actual = can_plant_flowers(vec![1, 0, 0, 0, 1], 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_can_plant_flowers_2() {
        let expected = false;
        let actual = can_plant_flowers(vec![1, 0, 0, 0, 0, 1], 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_can_plant_flowers_3() {
        let expected = true;
        let actual = can_plant_flowers(vec![0, 0, 1, 0, 1], 1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_can_plant_flowers_4() {
        let expected = true;
        let actual = can_plant_flowers(vec![0, 0, 1, 0, 1], 1);
        assert_eq!(expected, actual);
    }
}
