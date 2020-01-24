struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let s = 5_f64.sqrt();
        let res = (((1. + s) / 2.).powi(n + 1) - ((1. - s) / 2.).powi(n + 1)) / s;
        res.round() as i32
    }
}

fn main() {
    let input = 3;
    assert_eq!(3, Solution::climb_stairs(input));
}