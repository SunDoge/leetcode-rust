use std::collections::BTreeSet;

pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    if k < 1 || t < 0 {
        return false;
    }

    let t = t as i64;
    let k = k as usize;

    let mut set: BTreeSet<i64> = BTreeSet::new();

    for (i, x) in nums.iter().enumerate() {
        let x = *x as i64;
        let mut r = set.range((x - t)..=(x + t));
        match r.next() {
            Some(_) => return true,
            None => {
                set.insert(x);
                if i >= k {
                    set.remove(&(nums[i - k] as i64));
                }
            }
        }
    }

    false
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    let k = 3;
    let t = 0;
    assert_eq!(contains_nearby_almost_duplicate(nums, k, t), true);

    let nums = vec![1, 5, 9, 1, 5, 9];
    let k = 2;
    let t = 3;
    assert_eq!(contains_nearby_almost_duplicate(nums, k, t), false);

    let nums = vec![0, 2147483647];
    let k = 1;
    let t = 2147483647;
    assert_eq!(contains_nearby_almost_duplicate(nums, k, t), true);
}
