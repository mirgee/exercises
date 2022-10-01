//Given a positive integer num, return the number of positive integers less than or equal to num
//whose digit sums are even.
//
//The digit sum of a positive integer is the sum of all its digits.
// Intuition here is that numbers have digitSum even and odd consecutively
// 
// 0 1 2 3 4 5 6 7 8 9 => 4 numbers with digit sum as even (all even numbers)
// 10 11 12 13 14 15 16 17 18 19 => 5 numbers with digit sum as even 11, 13, 15, 17, 19 (all odd numbers)
// 20 21 22 23 24 25 26 27 28 29 => 5 numbers with digit sum as even 20, 22, 24, 26, 28 (all even numbers)
// 30 31 32 33 34 35 36 37 38 39 => 5 numbers with digit sum as even 31, 33, 35, 37, 39 (all odd numbers)
// 
// So the evens can be given as num//2 in all the cases except when the number itself is even but digit sum is odd. Example:10, 12, 30
// In such cases the even numbers would less by 1 as that particular number is not a valid number.

fn main() {
    println!("Hello, world!");
}

fn digits(num: i32) -> Vec<i32> {
    let mut v = Vec::new();
    let mut n = num;
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }
    v
}

fn count_even(num: i32) -> i32 {
    if num % 2 != 0 {
        return num / 2
    }
    num / 2 - digits(num).iter().sum::<i32>() as i32 % 2
}

#[test]
fn test_count_even() {
    assert_eq!(count_even(4), 2);
    assert_eq!(count_even(30), 14);
}
