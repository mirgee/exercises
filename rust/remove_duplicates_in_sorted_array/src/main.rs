fn main() {
    println!("Hello, world!");
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 0;    
    let mut l = nums.len();
    while i < l {
        let j = i+1;
        while j < l && nums[i] == nums[j] {
            nums.remove(j);
            l -= 1;
        }
        i += 1;
    }
    return l as i32;
}

#[test]
fn test_remove_duplicates() {
    let mut nums: Vec<i32> = Vec::from([0,0,1,1,1,2,2,3,3,4]);
    let mut expected_nums: Vec<i32> = Vec::from([0,1,2,3,4]);

    let k = remove_duplicates(&mut nums);

    assert_eq!(k, expected_nums.len() as i32);
    for i in 0..k {
        assert_eq!(nums.get(i as usize), expected_nums.get(i as usize));
    }
}

#[test]
fn test_remove_duplicates_1() {
    let mut nums: Vec<i32> = Vec::from([]);
    let mut expected_nums: Vec<i32> = Vec::from([]);

    let k = remove_duplicates(&mut nums);

    assert_eq!(k, expected_nums.len() as i32);
    for i in 0..k {
        assert_eq!(nums.get(i as usize), expected_nums.get(i as usize));
    }
}
