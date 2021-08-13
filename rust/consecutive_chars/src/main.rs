fn main() {
    println!("Hello, world!");
}

fn max_power(s: String) -> i32 {
    let mut longest: i32 = 1;
    let mut count = 1;
    let mut chars = s.chars();
    let mut prev = chars.next().unwrap();
    let mut curr = chars.next();
    while curr.is_some() {
        let _curr = curr.unwrap();
        if _curr == prev {
            count += 1;
        } else {
            count = 1;
        }
        longest = std::cmp::max(count, longest);
        prev = _curr;
        curr = chars.next();
    }
    longest
}

#[test]
fn test_max_power() {
    assert_eq!(max_power("leetcode".to_string()), 2);
    assert_eq!(max_power("abbcccddddeeeeedcba".to_string()), 5);
    assert_eq!(max_power("triplepillooooow".to_string()), 5);
    assert_eq!(max_power("hooraaaaaaaaaaay".to_string()), 11);
    assert_eq!(max_power("tourist".to_string()), 1);
    assert_eq!(max_power("t".to_string()), 1);
}
