struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        Self::add(&mut digits, 1);
        digits
    }

    fn add(digits: &mut Vec<i32>, digit: i32) {
        let mut c = digit;

        for d in digits.iter_mut().rev() {
            *d += c;
            c = *d / 10;
            *d = *d % 10;
        }

        if c > 0 {
            digits.insert(0, 1);
        }
    }
}


fn main() {
    let input = vec![1, 2, 3];
    assert_eq!(vec![1, 2, 4], Solution::plus_one(input));
}