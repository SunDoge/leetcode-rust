struct Solution;

impl Solution {
    // O(nlogn)
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let length = nums.len();
        nums[length / 2]
    }

    // O(n)
    pub fn majority_element1(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |(majority, votes), &x| {
                if votes == 0 {
                    (x, 1)
                } else if majority == x {
                    (majority, votes + 1)
                } else {
                    (majority, votes - 1)
                }
            })
            .0
    }
}

fn main() {
    let input = vec![3, 2, 3];
    assert_eq!(Solution::majority_element(input), 3);
    let input = vec![2, 2, 1, 1, 1, 2, 2];
    assert_eq!(Solution::majority_element(input), 2);

    let input = vec![3, 2, 3];
    assert_eq!(Solution::majority_element1(input), 3);
    let input = vec![2, 2, 1, 1, 1, 2, 2];
    assert_eq!(Solution::majority_element1(input), 2);
}
