// https://leetcode.com/problems/longest-common-prefix/description/
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let first = strs.get(0).unwrap().to_owned();
    strs.iter().skip(1).fold(first, |acc, c| {
        acc
            .chars()
            .zip(c.chars())
            .take_while(|(f, s)| f == s)
            .map(|(f, _)| f)
            .collect()
    })

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix_0() {
        let expected = "fl".to_string();
        let actual = longest_common_prefix(["flower","flow","flight"].map(str::to_string).to_vec());
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn test_longest_common_prefix_1() {
        let expected = "".to_string();
        let actual = longest_common_prefix(["dog","racecar","car"].map(str::to_string).to_vec());
        assert_eq!(expected, actual);
    }
}
