pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut res: Vec<i32> = Vec::new();
    let nums_len = nums.len();
    let mut i = 0;
    while i < nums_len {
        let count = nums.iter().filter(|&n| *n == nums[i]).count();
        if count > 2 {
            res.push(nums[i]);
            res.push(nums[i + 1]);
            i += count - 1;
        } else {
            res.push(nums[i]);
        }
        i += 1;
    }
    *nums = res;
    nums.len() as i32
}

fn main() {
    let mut nums = vec![0,0,1,1,1,1,2,3,3];
    let result = remove_duplicates(&mut nums);
    println!("Result: {}", result);
}