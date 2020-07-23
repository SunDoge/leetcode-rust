pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums[0];
    }

    let mut max_so_far = nums[0];
    let mut max_ending_here = nums[0];

    for x in nums[1..nums.len()].iter() {
        max_ending_here = (max_ending_here + *x).max(*x);
        max_so_far = max_so_far.max(max_ending_here);
    }

    max_so_far
}

// 这根本不easy。。。
// https://leetcode.com/problems/maximum-subarray/discuss/20211/Accepted-O(n)-solution-in-java
fn main() {
    let nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4].to_vec();
    assert_eq!(max_sub_array(nums), 6);
}
