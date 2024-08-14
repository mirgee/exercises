// https://leetcode.com/problems/duplicate-zeros/description/

fn main() {
    println!("Hello, world!");
}

fn duplicate_zeros(arr: &mut Vec<i32>) {
    let len = arr.len();
    let mut arr_new: Vec<i32> = std::vec::Vec::new();
    for (i, arr) in arr.iter().enumerate() {
        arr_new.push(arr.clone());
        if *arr == 0 {
            arr_new.push(arr.clone());
        }
    }
    arr_new.resize(len, 0);
    *arr = arr_new;
}

fn duplicate_zeros_1(nums: &mut Vec<i32>) {
    let mut counter: usize = nums.iter().map(|&x| if x == 0 { 2 } else { 1 }).sum();
    for ind in (0..nums.len()).rev() {
        let num = nums[ind];
        counter -= if num == 0 { 2 } else { 1 };
        if counter < nums.len() {
            nums[counter] = num;
            if num == 0 && counter + 1 < nums.len() {
                nums[counter + 1] = num;
            }
        }
    }
}

#[test]
fn test_duplicate_zeros() {
    let mut vec = std::vec::Vec::<i32>::from([1,0,2,3,0,4,5,0]);
    duplicate_zeros_1(&mut vec);
    println!("{:?}", vec);
}

#[test]
fn test_duplicate_zeros_1() {
    let mut vec = std::vec::Vec::<i32>::from([2,3,0,5,0,0,1,0]);
    duplicate_zeros_1(&mut vec);
    println!("{:?}", vec);
}
