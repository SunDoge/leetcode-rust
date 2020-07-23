pub fn count_and_say(n: i32) -> String {
    if n == 1 {
        return "1".to_string();
    }
    let prev = count_and_say(n - 1);

    let number_and_count: Vec<(u32, usize)> =
        prev.chars()
            .map(|c| c.to_digit(10).unwrap())
            .fold(Vec::new(), |mut acc, x| {
                match acc.last_mut() {
                    Some((number, count)) => {
                        if *number == x {
                            *count += 1;
                        } else {
                            acc.push((x, 1))
                        }
                    }
                    None => acc.push((x, 1)),
                }

                acc
            });

    number_and_count
        .iter()
        .fold(String::new(), |mut acc, (number, count)| {
            acc.push_str(&format!("{}{}", count, number));
            acc
        })
}

// 这道题表述不清，其实就是下一个序列是上一个序列的翻译
fn main() {
    assert_eq!(count_and_say(4), "1211".to_string());
}
