pub struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = board[0].len();
        let mut next_board = board.clone();

        // 遍历整个数组
        for i in 0..m {
            for j in 0..n {
                let mut live_neighbors = 0;

                // 计算活细胞的邻居数
                for di in [-1, 0, 1] {
                    for dj in [-1, 0, 1] {
                        if di == 0 && dj == 0 {
                            continue;
                        }

                        let neighbor_i = (i as isize + di) as usize;
                        let neighbor_j = (j as isize + dj) as usize;

                        if neighbor_i < m && neighbor_j < n && board[neighbor_i][neighbor_j] == 1 {
                            live_neighbors += 1;
                        }
                    }
                }

                // 应用生命游戏的规则
                if board[i][j] == 1 {
                    if live_neighbors < 2 || live_neighbors > 3 {
                        next_board[i][j] = 0;
                    }
                } else {
                    if live_neighbors == 3 {
                        next_board[i][j] = 1;
                    }
                }
            }
        }

        *board = next_board;
    }
}
