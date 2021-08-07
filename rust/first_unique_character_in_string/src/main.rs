use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn first_non_repeating(s: String) -> i32 {
    let mut seen = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        if let Some(_prev_index) = seen.get(&c.into()).cloned() {
            seen.insert(c, i32::MAX);
        } else {
            seen.insert(c, i as i32);
        }
    }
    seen.values().min().cloned().unwrap_or(-1 as i32)
}

#[test]
fn test_first_non_repeating() {
    assert_eq!(2, first_non_repeating("loveleetcode".to_string()));
}

