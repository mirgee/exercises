use std::collections::HashMap;

pub fn ransom_note(ransom_note: String, magazine: String) -> bool {
    let mut hm = HashMap::new();

    for c in magazine.chars() {
        hm.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    for c in ransom_note.chars() {
        match hm.get_mut(&c) {
            Some(v) if *v > 0 => *v -= 1,
            _ => return false
        }
    }
    return true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ransom_note_0() {
        let expected = false;
        let actual = ransom_note("a".to_string(), "b".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_ransom_note_1() {
        let expected = false;
        let actual = ransom_note("aa".to_string(), "ab".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_ransom_note_2() {
        let expected = true;
        let actual = ransom_note("aa".to_string(), "aab".to_string());
        assert_eq!(expected, actual);
    }
}
