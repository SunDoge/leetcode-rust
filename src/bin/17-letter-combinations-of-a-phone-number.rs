pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        // [] -> []
        return Vec::new();
    }

    digits
        .chars()
        .map(|ch| match ch {
            '2' => "abc",
            '3' => "def",
            '4' => "ghi",
            '5' => "jkl",
            '6' => "mno",
            '7' => "pqrs",
            '8' => "tuv",
            '9' => "wxyz",
            _ => "",
        })
        .fold(vec!["".to_string()], |acc, x| {
            acc.iter()
                .map(|item| {
                    let mut res = Vec::new();
                    for ch in x.chars() {
                        res.push(format!("{}{}", item, ch));
                    }
                    res
                })
                .flatten()
                .collect::<Vec<String>>()
        })
}

fn main() {
    let input = "23".to_string();
    let output: Vec<String> = ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].iter().map(|x|x.to_string()).collect();
    println!("{:?}", letter_combinations(input.clone()));
    assert_eq!(letter_combinations(input.clone()), output);
}
