struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for i in 0..(n / 2) {
            matrix.swap(i, n - i - 1);
        }

        // TODO: better transpose?
        for i in 0..n {
            for j in (i + 1)..n {
                let temp = matrix[j][i];
                matrix[j][i] = matrix[i][j];
                matrix[i][j] = temp;
            }
        }
    }
}

fn main() {
    let mut input = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];

    let output = vec![
        vec![7, 4, 1],
        vec![8, 5, 2],
        vec![9, 6, 3]
    ];

    Solution::rotate(&mut input);

    assert_eq!(output, input);
}