pub fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    let mut x1 = x;
    let mut x2 = 0;

    while x2 < x1 {
        let rest = x1 % 10;
        x2 = x2 * 10 + rest;
        x1 /= 10;
    }

    x1 == x2 || x2 / 10 == x1
}

fn main() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(10), false);
}
