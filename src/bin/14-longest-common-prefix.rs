pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    // 找到最短字符
    let shortest = strs.iter().map(|s| s.len()).min().unwrap_or(0);

    let mut index = 0;

    'index: while index < shortest {
        let ch = strs[0].as_bytes()[index];
        for s in strs.iter() {
            let chi = s.as_bytes()[index];

            if chi != ch {
                break 'index;
            }
        }

        index += 1;
    }

    strs[0][0..index].to_string()
}

fn main() {
    let strs: Vec<String> = ["flower", "flow", "flight"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let output = "fl".to_string();
    assert_eq!(longest_common_prefix(strs), output);

    let strs: Vec<String> = ["dog", "racecar", "car"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let output = "".to_string();
    assert_eq!(longest_common_prefix(strs), output);
}
