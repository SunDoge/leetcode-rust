pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut x1 = std::i32::MAX;
    let mut x2 = std::i32::MAX;

    for x in nums {
        if x <= x1 {
            x1 = x;
        } else if x <= x2 {
            x2 = x;
        } else {
            return true;
        }
    }

    false
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    assert_eq!(increasing_triplet(nums), true);

    let nums = vec![5, 4, 3, 2, 1];
    assert_eq!(increasing_triplet(nums), false);
}
