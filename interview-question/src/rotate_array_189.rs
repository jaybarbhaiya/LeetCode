fn rotate(nums: &mut Vec<i32>, k: i32) {
    let nums_len = nums.len();
    let nums_copy = nums.clone();
    for i in 0..nums_len {
        nums[(i + k as usize) % nums_len] = nums_copy[i];
    }
}

pub fn run() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    rotate(&mut nums, k);
    println!("{:?}", nums);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
}
