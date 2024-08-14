// https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/
fn adjacent_duplicates(s: String) -> String {
    let mut stack: Vec<char> = Vec::with_capacity(s.len());
    for x in s.chars() {
        if let Some(last) = stack.last() {
            if last.to_string() == x.to_string() {
                stack.pop().unwrap();
            } else {
                stack.push(x);
            }
        } else {
            stack.push(x);
        }
    }
    stack.iter().collect()
}

pub fn adjacent_duplicates_1(s: String) -> String {
    s.chars()
        .fold(Vec::new(), |mut stack, c| {
            if stack.last() == Some(&c) {
                stack.pop();
            } else {
                stack.push(c);
            }
            stack
        })
        .iter()
        .collect()
}

#[test]
fn test_adjacent_duplicates() {
    assert_eq!(adjacent_duplicates("abbaca".to_string()), "ca");
    assert_eq!(adjacent_duplicates("azxxzy".to_string()), "ay");
}
