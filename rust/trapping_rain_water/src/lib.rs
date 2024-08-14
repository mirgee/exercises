pub fn trapping_rain_water(height: Vec<i32>) -> i32 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trapping_rain_water_0() {
        let expected = 6;
        let actual = trapping_rain_water(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_trapping_rain_water_1() {
        let expected = 9;
        let actual = trapping_rain_water(vec![4, 2, 0, 3, 2, 5]);
        assert_eq!(expected, actual);
    }
}
