pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut left = vec![1i32; nums.len()];

    for i in 1..nums.len() {
        left[i] = nums[i - 1] * left[i - 1];
    }

    let mut right = nums[nums.len() - 1];
    for i in (0..nums.len() - 1).rev() {
        // println!("{}", i);
        left[i] *= right;
        right *= nums[i];
    }

    left
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let out = vec![24, 12, 8, 6];
    assert_eq!(product_except_self(nums), out);
}
