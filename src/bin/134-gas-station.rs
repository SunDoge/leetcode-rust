struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut sum = 0;
        let mut j = -1;

        for (i, (g, c)) in gas.iter().zip(cost.iter()).enumerate() {
            sum += g - c;
            total += g - c;
            if sum < 0 {
                j = i as i32;
                sum = 0;
            }
        }

        if total >= 0 {
            j + 1
        } else {
            -1
        }
    }
}

fn main() {
    let gas = vec![1, 2, 3, 4, 5];
    let cost = vec![3, 4, 5, 1, 2];
    assert_eq!(3, Solution::can_complete_circuit(gas, cost));
}