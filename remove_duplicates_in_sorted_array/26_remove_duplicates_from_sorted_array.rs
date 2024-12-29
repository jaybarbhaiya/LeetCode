pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

fn main() {
    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
    let result = remove_duplicates(&mut nums);
    println!("Result: {}", result);
}