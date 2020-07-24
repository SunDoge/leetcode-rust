pub fn my_sqrt(x: i32) -> i32 {
    let x1 = x as i64;
    let mut r = x1;
    // 把这个判断条件换一下就不用i64
    // 因为我们知道Newton算法是单调的
    while r * r > x1 {
        r = (r + x1 / r) / 2;
    }

    r as i32
}

// https://leetcode.com/problems/sqrtx/discuss/25057/3-4-short-lines-Integer-Newton-Every-Language
fn main() {
    assert_eq!(my_sqrt(4), 2);
    assert_eq!(my_sqrt(8), 2);
}
