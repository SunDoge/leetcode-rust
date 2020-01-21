struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut used = [false; 9];

        for i in 0..9 {
            used.iter_mut().for_each(|x| {
                *x = false;
            });


            for j in 0..9 {
                // check row
                if !Self::check(board[i][j], &mut used) {
                    return false;
                }
            }

            used.iter_mut().for_each(|x| {
                *x = false;
            });

            for j in 0..9 {
                if !Self::check(board[j][i], &mut used) {
                    return false;
                }
            }
        }

        for r in 0..3 {
            for c in 0..3 {
                used.iter_mut().for_each(|x| {
                    *x = false;
                });

                for i in (r * 3)..(r * 3 + 3) {
                    for j in (c * 3)..(c * 3 + 3) {
                        if !Self::check(board[i][j], &mut used) {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    fn check(ch: char, used: &mut [bool]) -> bool {
        if ch == '.' {
            return true;
        }
        if used[(ch as u8 - '1' as u8) as usize] {
            return false;
        }
        used[(ch as u8 - '1' as u8) as usize] = true;
        true
    }
}

fn main() {
    let input = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ];
    assert_eq!(true, Solution::is_valid_sudoku(input));
}