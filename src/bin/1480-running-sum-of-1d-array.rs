pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().fold(Vec::new(), |mut acc, x| {
        acc.push(acc.last().unwrap_or(&0) + x);
        acc
    })
}

fn main() {
    let nums = [1, 2, 3, 4].to_vec();
    let output = [1, 3, 6, 10].to_vec();
    assert_eq!(running_sum(nums), output);

    let nums = [3,1,2,10,1].to_vec();
    let output = [3,4,6,16,17].to_vec();
    assert_eq!(running_sum(nums), output);
}
