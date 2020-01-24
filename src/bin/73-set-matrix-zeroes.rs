struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut row_contains_zero = false;
        let mut col_contains_zero = false;

        for j in 0..n {
            if matrix[0][j] == 0 {
                row_contains_zero = true;
                break;
            }
        }

        for i in 0..m {
            if matrix[i][0] == 0 {
                col_contains_zero = true;
                break;
            }
        }

        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }

        for i in 1..m {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if row_contains_zero {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }

        if col_contains_zero {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
}

fn main() {
    let mut input = vec![
        vec![0, 1, 2, 0],
        vec![3, 4, 5, 2],
        vec![1, 3, 1, 5]
    ];

    let output = vec![
        vec![0, 0, 0, 0],
        vec![0, 4, 5, 0],
        vec![0, 3, 1, 0]
    ];

    Solution::set_zeroes(&mut input);
    assert_eq!(output, input);
}