// O(nlogn)
pub fn my_pow(x: f64, n: i32) -> f64 {
    if n < 0 {
        1.0 / power(x, n)
    } else {
        power(x, n)
    }
}

pub fn power(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }
    let v = power(x, n / 2);
    if n % 2 == 0 {
        v * v
    } else {
        v * v * x
    }
}

fn equal(x: f64, y: f64) {
    let eps = 0.00000001;
    assert!((x - y).abs() < eps);
}

fn main() {
    let (x, n) = (2.00000, 10);
    equal(my_pow(x, n), 1024.0);

    let (x, n) = (2.10000, 3);
    equal(my_pow(x, n), 9.26100);

    let (x, n) = (2.00000, -2);
    equal(my_pow(x, n), 0.25000);
}
