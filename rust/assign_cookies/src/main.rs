// https://leetcode.com/problems/assign-cookies

fn assign_cookies(g: Vec<i32>, s: Vec<i32>) -> i32 {
    let mut s = s.clone();
    let mut g = g.clone();
    s.sort(); g.sort();
    let mut s = s.iter();
    g.into_iter()
        .take_while(|g| s.any(|s| g <= s))
        .count() as i32
}

#[test]
fn test_assign_cookies() {
    assert_eq!(assign_cookies(vec![1,2,3], vec![1,1]), 1);
    assert_eq!(assign_cookies(vec![1,2], vec![1,2,3]), 2);
}
