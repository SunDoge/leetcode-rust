use std::collections::BTreeMap;

// pub fn length_of_longest_substring(s: String) -> i32 {
//     let mut max_length = 0;
//     let mut start_index = 0;
//     let mut counter: BTreeMap<char, usize> = BTreeMap::new();

//     for (index, ch) in s.char_indices() {
//         // 重复的char不算，所以记录下一个index
//         if let Some(last_index) = counter.insert(ch, index + 1) {
//             start_index = start_index.max(last_index);
//         }

//         max_length = max_length.max(index + 1 - start_index);
//     }

//     max_length as i32
// }

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut index = vec![0usize; 128];
    let mut ans = 0;
    let mut start_index = 0;

    for (i, ch) in s.char_indices() {
        start_index = start_index.max(index[ch as usize]);
        ans = ans.max(i - start_index + 1); // 计算长度，下一位减开始位
        index[ch as usize] = i + 1; // 记录下一位index
    }

    ans as i32
}

fn main() {
    let input = "abcabcbb";
    let output = 3;
    assert_eq!(length_of_longest_substring(input.to_string()), output);

    let input = "bbbbb";
    let output = 1;
    assert_eq!(length_of_longest_substring(input.to_string()), output);

    let input = "pwwkew";
    let output = 3;
    assert_eq!(length_of_longest_substring(input.to_string()), output);

    let input = " ";
    let output = 1;
    assert_eq!(length_of_longest_substring(input.to_string()), output);

    let input = "dvdf";
    let output = 3;
    assert_eq!(length_of_longest_substring(input.to_string()), output);
}
