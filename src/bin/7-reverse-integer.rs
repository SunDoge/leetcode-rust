pub fn reverse(x: i32) -> i32 {
    let mut x1 = x;
    let mut x2 = 0;

    while x1 != 0 {
        let number = x1 % 10;
        x1 /= 10;

        if x2 > std::i32::MAX / 10 || (x2 == std::i32::MAX / 10 && number > std::i32::MAX % 10) {
            return 0;
        }

        if x2 < std::i32::MIN / 10 || (x2 == std::i32::MIN / 10 && number < std::i32::MIN % 10) {
            return 0;
        }

        x2 = x2 * 10 + number;
    }
    x2
}

fn main() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(-123), -321);
    assert_eq!(reverse(120), 21);
}
