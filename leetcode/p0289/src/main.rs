fn main() {
    println!("Hello, world!");
}

pub struct Solution {}

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let (height, width) = (board.len(), board[0].len());
        let neighbors = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for i in 0..height {
            for j in 0..width {
                let mut live = 0;
                for offset in neighbors.iter() {
                    if (offset.0 < 0 && i == 0)
                        || (offset.0 > 0 && i == height - 1)
                        || (offset.1 < 0 && j == 0)
                        || (offset.1 > 0 && j == width - 1)
                    {
                        continue;
                    }
                    let v = board[(i as i32 + offset.0) as usize][(j as i32 + offset.1) as usize];
                    if v == 1 || v == 3 {
                        live += 1;
                    }
                }
                if board[i][j] == 1 && (live < 2 || live > 3) {
                    // go die
                    board[i][j] = 3;
                } else if board[i][j] == 0 && live == 3 {
                    // go live
                    board[i][j] = 2;
                }
            }
        }

        for i in 0..height {
            for j in 0..width {
                if board[i][j] == 2 {
                    board[i][j] = 1;
                } else if board[i][j] == 3 {
                    board[i][j] = 0;
                }
            }
        }
    }
}
