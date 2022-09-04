pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    let l = s.len();
    if l == 0 {
        return vec![vec![]];
    }
    let mut res = vec![];
    let mut i = 0;
    let mut j = 1;
    while j < l {
        let a = s.chars().nth(i).unwrap();
        let mut b = s.chars().nth(j).unwrap();
        while a == b && j < l {
            j += 1;
            b = if let Some(b) = s.chars().nth(j) {
                b
            } else {
                break
            }
        }
        if j - i >= 3 {
            res.push(vec![i as i32, (j-1) as i32]);
        }
        i = j;
        j += 1;
    }
    res
}

#[test]
fn test_large_group_positions() {
    assert_eq!(large_group_positions("abbxxxxzzy".to_string()), vec![vec![3,6]]);
    println!("{:?}", large_group_positions("abc".to_string()));
    println!("{:?}", large_group_positions("aa".to_string()));
    assert_eq!(large_group_positions("abbxxxxzzy".to_string()), vec![vec![3,6]]);
    assert_eq!(large_group_positions("abcdddeeeeaabbbcd".to_string()), vec![vec![3,5],vec![6,9],vec![12,14]]);
}
