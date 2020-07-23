pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
        return 0;
    }

    let haystack_len = haystack.len();
    let needle_len = needle.len();

    if needle_len > haystack_len {
        return -1;
    }

    for i in 0..=(haystack_len - needle_len) {
        let x = &haystack[i..i + needle_len];
        if x == needle {
            return i as i32;
        }
    }

    -1
}

fn main() {
    let haystack = "hello".to_string();
    let needle = "ll".to_string();
    assert_eq!(str_str(haystack, needle), 2);

    let haystack = "aaaaa".to_string();
    let needle = "bba".to_string();
    assert_eq!(str_str(haystack, needle), -1);
}
