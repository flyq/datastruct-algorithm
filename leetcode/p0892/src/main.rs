fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }
        let mut sum = 0;
        let mut same = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                sum += grid[i][j];
                if grid[i][j] != 0 {
                    same += grid[i][j]-1;
                }
                if grid.get(i-1) != None {
                    if grid[i-1][j] < grid[i][j] {
                        same += grid[i-1][j];
                    } else {
                        same += grid[i][j];
                    }
                }
                if grid[i].get(j-1) != None {
                    if grid[i][j-1] < grid[i][j] {
                        same += grid[i][j-1];
                    } else {
                        same += grid[i][j];
                    }
                }
            }
        }
        6*sum - 2*same
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
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
