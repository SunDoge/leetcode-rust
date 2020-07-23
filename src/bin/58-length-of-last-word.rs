pub fn length_of_last_word(s: String) -> i32 {
    let splits = s.trim().split(" ");
    let last_word = splits.last();
    if let Some(w) = last_word {
        w.len() as i32
    } else {
        0
    }
}

pub fn length_of_last_word1(s: String) -> i32 {
    s.chars()
        .rev()
        .skip_while(|c| *c == ' ')
        .take_while(|c| *c != ' ')
        .count() as i32
}

fn main() {
    let s = "Hello World".to_string();
    assert_eq!(length_of_last_word(s), 5);

    let s = "Hello World".to_string();
    assert_eq!(length_of_last_word1(s), 5);
}
