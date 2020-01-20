struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut s = String::with_capacity(n as usize);
        for i in 0..n {
            s.push_str(&(i + 1).to_string());
        }
        Self::kth_permutation(&mut s, n, k)
    }

    fn factorial(n: i32) -> i32 {
        let mut result = 1;
        for i in 1..(n + 1) {
            result *= i;
        }
        result
    }

    fn kth_permutation(s: &mut String, n: i32, k: i32) -> String {
        let mut result = String::with_capacity(n as usize);

        let mut base = Self::factorial(n - 1);

        let mut k = k - 1;

        let mut i = n - 1;

        while i > 0 {
            let idx = (k / base) as usize;
            let a = s.chars().nth(idx).unwrap();
            result.push(a);
            s.remove(idx);

            k %= base;
            base /= i;
            i -= 1;
        }
        result.push(s.chars().nth(0).unwrap());
        result
    }
}

fn main() {
    let n = 3;
    let k = 3;
    assert_eq!("213", Solution::get_permutation(n, k));
}