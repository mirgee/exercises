fn main() {
    println!("Hello, world!");
}

fn truncate_sentence(s: String, k: i32) -> String {
    s.split(" ").take(k as usize).collect::<Vec<_>>().join(" ").to_string()
}

#[test]
fn test_truncate_sentence() {
    assert_eq!(truncate_sentence("Hello how are you Contestant".to_string(), 4), "Hello how are you".to_string());
    assert_eq!(truncate_sentence("What is the solution to this problem".to_string(), 4), "What is the solution".to_string());
    assert_eq!(truncate_sentence("chopper is not a tanuki".to_string(), 5), "chopper is not a tanuki".to_string());
}
