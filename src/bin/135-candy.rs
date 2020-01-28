struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut increment = vec![0; n];

        let mut inc = 1;
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                increment[i] = inc.max(increment[i]);
                inc += 1;
            } else {
                inc = 1;
            }
        }

        inc = 1;
        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                increment[i] = inc.max(increment[i]);
                inc += 1;
            } else {
                inc = 1;
            }
        }

        increment.iter().sum::<i32>() + n as i32
    }
}

fn main() {
    let input = vec![1, 2, 2];
    assert_eq!(4, Solution::candy(input));
    let input = vec![1, 0, 2];
    assert_eq!(5, Solution::candy(input));
}