// A string s can be partitioned into groups of size k using the following procedure:
// 
// The first group consists of the first k characters of the string, the second group consists of the next k characters of the string, and so on. Each character can be a part of exactly one group.
// For the last group, if the string does not have k characters remaining, a character fill is used to complete the group.
// Note that the partition is done so that after removing the fill character from the last group (if it exists) and concatenating all the groups in order, the resultant string should be s.
// 
// Given the string s, the size of each group k and the character fill, return a string array denoting the composition of every group s has been divided into, using the above procedure.


fn main() {
    println!("Hello, world!");
}

fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut interm: Vec<char> = Vec::new();
    for c in s.chars().into_iter() {
        if interm.len() as i32 == k {
            res.push(interm.into_iter().collect());
            interm = Vec::new();
        }
        interm.push(c.clone());
    }
    if !interm.is_empty() {
        while (interm.len() as i32) < k {
            interm.push(fill);
        }
        res.push(interm.into_iter().collect());
    }
    res
}

#[test]
fn test_divide_string() {
    assert_eq!(divide_string("abcdefghi".to_string(), 3, "x".chars().last().unwrap()), vec!["abc","def","ghi"].into_iter().map(|s| s.to_string()).collect::<Vec<String>>());
    assert_eq!(divide_string("abcdefghij".to_string(), 3, "x".chars().last().unwrap()), vec!["abc","def","ghi", "jxx"].into_iter().map(|s| s.to_string()).collect::<Vec<String>>());
}
