use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let symbols = [
        (b'I', 1),
        (b'V', 5),
        (b'X', 10),
        (b'L', 50),
        (b'C', 100),
        (b'D', 500),
        (b'M', 1000),
    ];

    let mut symbol_to_value = HashMap::with_capacity(symbols.len());
    for (k, v) in symbols.iter() {
        symbol_to_value.insert(*k, *v);
    }

    // let s1: &str = &s;
    let bytes = s.as_bytes();

    let mut result = 0;
    for i in 0..s.len() - 1 {
        if symbol_to_value[&bytes[i]] < symbol_to_value[&bytes[i + 1]] {
            result -= symbol_to_value[&bytes[i]];
        } else {
            result += symbol_to_value[&bytes[i]];
        }
    }

    result += symbol_to_value[&bytes.last().unwrap()];
    result
}

pub fn roman_to_int_fold(s: String) -> i32 {
    s.chars()
        .fold(('~', 0), |(prev, mut acc), c| {
            match (prev, c) {
                (_, 'I') => acc += 1,
                ('I', 'V') => acc += 4 - 1,
                (_, 'V') => acc += 5,
                ('I', 'X') => acc += 9 - 1,
                (_, 'X') => acc += 10,
                ('X', 'L') => acc += 40 - 10,
                (_, 'L') => acc += 50,
                ('X', 'C') => acc += 90 - 10,
                (_, 'C') => acc += 100,
                ('C', 'D') => acc += 400 - 100,
                (_, 'D') => acc += 500,
                ('C', 'M') => acc += 900 - 100,
                (_, 'M') => acc += 1000,
                (_, _) => {}
            };
            (c, acc)
        })
        .1
}

fn main() {
    let s = "III".to_string();
    assert_eq!(roman_to_int(s), 3);

    let s = "IV".to_string();
    assert_eq!(roman_to_int(s), 4);

    let s = "III".to_string();
    assert_eq!(roman_to_int_fold(s), 3);

    let s = "IV".to_string();
    assert_eq!(roman_to_int_fold(s), 4);
}
