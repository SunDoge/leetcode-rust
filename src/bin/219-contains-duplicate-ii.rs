use std::collections::HashMap;

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    if nums.len() < 2 {
        return false;
    }

    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

    let mut min = std::usize::MAX;

    for (i, x) in nums.iter().enumerate() {
        let value = map.get(x);

        if let Some(&pre_index) = value {
            let gap = i - pre_index;
            min = min.min(gap);
        }

        map.insert(*x, i);
    }

    min <= k as usize
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    let k = 3;
    assert_eq!(contains_nearby_duplicate(nums, k), true);

    let nums = vec![1, 2, 3, 1, 2, 3];
    let k = 2;
    assert_eq!(contains_nearby_duplicate(nums, k), false);
}
