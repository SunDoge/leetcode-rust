struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut low = 0i32;
        let mut high = (nums.len() - 1) as i32;

        while low <= high {
            let mid = (high + low) / 2;

            if nums[mid as usize] == target {
                // println!("{:?}", nums);
                return mid;
            } else if nums[mid as usize] > target {
                high = mid - 1;
            } else if nums[mid as usize] < target {
                low = mid + 1;
            }
        }

        low as i32
    }
}
fn main() {
    let nums = [1, 3, 5, 6].to_vec();
    assert_eq!(Solution::search_insert(nums, 5), 2);

    let nums = [1, 3, 5, 6].to_vec();
    assert_eq!(Solution::search_insert(nums, 2), 1);

    let nums = [1, 3, 5, 6].to_vec();
    assert_eq!(Solution::search_insert(nums, 0), 0);
}
