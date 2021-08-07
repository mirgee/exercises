// 680
fn main() {
    println!("Hello, world!");
}

fn check_rest(c: &[u8], mut l: usize, mut r: usize) -> bool {
    while l < r {
        if c[l] != c[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    return true;
}

pub fn valid_palindrome(s_: String) -> bool {
    let s = s_.as_bytes();
    let mut i = 0;
    let mut j = s_.len()-1;
    while i < j {
        if s[i] == s[j] {
            i += 1;
            j -= 1;
        } else {
            return check_rest(s, i+1, j) || check_rest(s, i, j-1)
        }
    }
    true
}

#[test]
fn test_valid_palindrome() {
    let s = "abc".to_string();
    assert!(!valid_palindrome(s));
    let s = "aba".to_string();
    assert!(valid_palindrome(s));
    let s = "abca".to_string();
    assert!(valid_palindrome(s));
    let s = "abcda".to_string();
    assert!(!valid_palindrome(s));
    let s = "mabkbapm".to_string();
    assert!(valid_palindrome(s));
    let s = "aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga".to_string();
    assert!(valid_palindrome(s));
}
