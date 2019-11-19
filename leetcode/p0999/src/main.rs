fn main() {
    println!("Hello, world!");
    let a = vec![vec!['.','.','.','.','.','.','.','.'],vec!['.','.','.','p','.','.','.','.'],vec!['.','.','.','R','.','.','.','p'],vec!['.','.','.','.','.','.','.','.'],vec!['.','.','.','.','.','.','.','.'],vec!['.','.','.','p','.','.','.','.'],vec!['.','.','.','.','.','.','.','.'],vec!['.','.','.','.','.','.','.','.']];
    println!("{}",Solution::num_rook_captures(a));
}
pub struct Solution{}

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {

        for i in 0..8  {
            for j in 0..8 {
                if board[i][j] == 'R' {
                    let x = i as i32;
                    let y = j as i32;
                    return Solution::cap(&board, x, y,1,0)+Solution::cap(&board,  x, y,0,-1)+Solution::cap(&board, x, y,-1,0)+Solution::cap(&board, x, y,0,1);
                }
            }
        }
        0
    }
    pub fn cap(a: &Vec<Vec<char>>,x: i32,y: i32,dx:i32,dy:i32) -> i32 {
        let mut x = x;
        let mut y = y;
        while x>=0 && x<8 && y>=0 && y<8 && a[x as usize][y as usize] != 'B' {
            if a[x as usize][y as usize] == 'p' {
                return 1;
            }
            x += dx;
            y += dy;
        }
        0
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
0 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
