pub fn is_valid(s: String) -> bool {
    if s.len() % 2 == 1 {
        return false;
    }

    let mut stack = Vec::new();

    for &ch in s.as_bytes() {
        match ch {
            b'(' => stack.push(b')'),
            b'[' => stack.push(b']'),
            b'{' => stack.push(b'}'),
            other => {
                if let Some(last) = stack.pop() {
                    if other != last {
                        return false;
                    }
                } else {
                    // 如果上来就是right，直接invalid
                    return false;
                }
            }
        }
    }

    // 必须所有括号都闭合
    stack.is_empty()
}

fn main() {
    let s = "()".to_string();
    assert_eq!(is_valid(s), true);
    let s = "()[]{}".to_string();
    assert_eq!(is_valid(s), true);

    let s = "([)]".to_string();
    assert_eq!(is_valid(s), false);
}
