fn main() {
    println!("Hello, world!");
}

fn min_operations(a: Vec<i32>) -> i32 {
    if a.len() == 1 {
        return 0
    }
    let mut incr = 0;
    let mut incr_last = a[0];
    for i in 1..a.len() {
        if incr_last < a[i] {
            incr_last = a[i];
            continue
        }
        incr_last += 1;
        incr += incr_last - a[i];
    }
    incr
}

#[test]
fn test_min_operations() {
    assert_eq!(min_operations(vec![1,1,1]), 3);
    assert_eq!(min_operations(vec![1,5,2,4,1]), 14);
    assert_eq!(min_operations(vec![8]), 0);
    assert_eq!(min_operations(vec![4881,2593,6819,9256,4135]), 7411);
}
