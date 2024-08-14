pub fn find_the_index_of_the_first_occurrence_in_a_string(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(idx) => idx as i32,
        None => -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_the_index_of_the_first_occurrence_in_a_string_0() {
        let expected = 0;
        let actual = find_the_index_of_the_first_occurrence_in_a_string(
            "sadbutsad".to_string(),
            "sad".to_string(),
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_the_index_of_the_first_occurrence_in_a_string_1() {
        let expected = -1;
        let actual = find_the_index_of_the_first_occurrence_in_a_string(
            "leetcode".to_string(),
            "leeto".to_string(),
        );
        assert_eq!(expected, actual);
    }
}
