pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let m = board.len();
    let n = board[0].len();

    // (-1, -1), ( 0, -1), ( 1, -1)
    // (-1,  0), ( 0,  0), ( 1,  0)
    // (-1,  1), ( 0,  1), ( 1,  1)
    // 逆时针
    let dx = vec![-1, -1, -1, 0, 1, 1, 1, 0];
    let dy = vec![-1, 0, 1, 1, 1, 0, -1, -1];

    // 0: dead -> dead
    // 1: live -> live
    // 2: live -> dead
    // 3: dead -> live

    for i in 0..m {
        for j in 0..n {
            let mut live = 0;

            for k in 0..8 {
                let x = i as i32 + dx[k];
                let y = j as i32 + dy[k];

                if x > -1
                    && x < m as i32
                    && y > -1
                    && y < n as i32
                    && (board[x as usize][y as usize] == 1 || board[x as usize][y as usize] == 2)
                {
                    live += 1;
                }
            }

            if board[i][j] == 0 && live == 3 {
                board[i][j] = 3;
            } else if board[i][j] == 1 && (live < 2 || live > 3) {
                board[i][j] = 2;
            }
        }
    }

    // for i in 0..m {
    //     for j in 0..m {
    //         board[i][j] %= 2;
    //     }
    // }
    board.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|x| *x %= 2);
    });
}

fn main() {
    let mut board: Vec<Vec<i32>> = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    let output: Vec<Vec<i32>> = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];
    game_of_life(&mut board);
    assert_eq!(board, output);
}
