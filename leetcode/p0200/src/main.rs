fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        if m == 0 || grid[0].len() == 0 {
            return 0;
        }
        let n = grid[0].len();
        let mut amount = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    amount += 1;
                    dfs(&mut grid, m, n, i, j);
                }
            }
        }
        amount
    }
}

fn dfs(grid: &mut Vec<Vec<char>>, m: usize, n: usize, i: usize, j: usize) {
    if i < 0 || i >= m || j < 0 || j >= n || grid[i][j] == '0' {
        return;
    }
    grid[i][j] = '0';
    dfs(grid, m, n, i - 1, j);
    dfs(grid, m, n, i + 1, j);
    dfs(grid, m, n, i, j - 1);
    dfs(grid, m, n, i, j + 1);
}
