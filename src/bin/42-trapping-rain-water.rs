struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut peak_index = 0;
        for i in 0..n {
            if height[i] > height[peak_index] {
                peak_index = i;
            }
        }

        let mut water = 0;
        let mut left_peak = 0;
        for i in 0..peak_index {
            if height[i] > left_peak {
                left_peak = height[i];
            } else {
                water += left_peak - height[i];
            }
        }

        let mut right_peak = 0;
        for i in (peak_index..n).rev() {
            if height[i] > right_peak {
                right_peak = height[i];
            } else {
                water += right_peak - height[i];
            }
        }

        water
    }
}

fn main() {
    let input = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    assert_eq!(6, Solution::trap(input));
}