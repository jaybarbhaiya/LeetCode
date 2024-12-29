
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let _ = nums1.split_off(m as usize);
    for i in 0..n {
        nums1.push(nums2[i as usize])
    }
    println!("{:?}", nums1.sort());
    println!("foo");
}

fn main() {
    let mut nums1 = vec![4, 0, 0, 0, 0, 0];
    let m = 1;
    let mut nums2 = vec![1, 2, 3, 5, 6];
    let n = 5;
    merge(&mut nums1, m, &mut nums2, n);
}
