pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();

    for (index, &num) in nums.iter().enumerate() {
        let key = target - num;
        if let Some(&value) = map.get(&key) {
            return vec![value as i32, index as i32];
        } else {
            map.insert(num, index);
        }
    }

    Vec::new()
}

fn main() {
    let nums = [2, 7, 11, 15].to_vec();
    let target = 9;
    let output = [0, 1].to_vec();
    assert_eq!(two_sum(nums, target), output);
}