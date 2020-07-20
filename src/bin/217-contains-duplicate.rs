use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {

    if nums.len() < 2 {
        return false;
    }

    let mut counter: HashSet<i32> = HashSet::with_capacity(nums.len());

    for x in nums {
        if !counter.insert(x) {
            return true;
        }
    }
    false
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    assert_eq!(contains_duplicate(nums), true);

    let nums = vec![1, 2, 3, 4];
    assert_eq!(contains_duplicate(nums), false);
}
