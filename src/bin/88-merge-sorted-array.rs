pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i1 = m as usize;
    let mut i2 = n as usize;

    while i1 > 0 && i2 > 0 {
        if nums1[i1 - 1] >= nums2[i2 - 1] {
            nums1[i1 + i2 - 1] = nums1[i1 - 1];
            i1 -= 1;
        } else {
            nums1[i1 + i2 - 1] = nums2[i2 - 1];
            i2 -= 1;
        }
    }

    if i2 > 0 {
        for i in 0..i2 {
            nums1[i] = nums2[i];
        }
    }
}

fn main() {
    let mut nums1 = [1, 2, 3, 0, 0, 0].to_vec();
    let m = 3;
    let mut nums2 = [2, 5, 6].to_vec();
    let n = 3;
    let output = [1, 2, 2, 3, 5, 6].to_vec();

    merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, output);
}
