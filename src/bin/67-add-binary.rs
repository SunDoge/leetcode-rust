pub fn add_binary(a: String, b: String) -> String {
    let mut carry = 0;
    let mut ai = a.len() as i32 - 1;
    let mut bi = b.len() as i32 - 1;
    let abytes = a.as_bytes();
    let bbytes = b.as_bytes();

    let mut s = String::new();

    while ai >= 0 || bi >= 0 || carry == 1 {
        carry += if ai >= 0 {
            let r = abytes[ai as usize] - b'0';
            ai -= 1;
            r
        } else {
            0
        };
        carry += if bi >= 0 {
            let r = bbytes[bi as usize] - b'0';
            bi -= 1;
            r
        } else {
            0
        };
        s.insert(0, ((carry % 2) as u8 + b'0') as char);
        carry /= 2;
    }
    s
}

fn main() {
    let a = "11".to_string();
    let b = "1".to_string();
    assert_eq!(add_binary(a, b), "100".to_string());

    let a = "1010".to_string();
    let b = "1011".to_string();
    assert_eq!(add_binary(a, b), "10101".to_string());
}
