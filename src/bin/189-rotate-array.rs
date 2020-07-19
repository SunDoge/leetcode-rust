pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize % nums.len();
    let length = nums.len();
    // 反转3次
    nums[0..(length - k)].reverse();
    nums[(length - k)..length].reverse();
    nums.reverse();
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    let output = vec![5, 6, 7, 1, 2, 3, 4];

    rotate(&mut nums, k);
    assert_eq!(nums, output);
}
