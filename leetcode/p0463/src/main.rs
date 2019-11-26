fn main() {
    println!("Hello, world!");
}
pub struct Solution {}

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut edge = 0;
        let col = grid[0].len();
        for i in 0..grid.len()*col {
            if grid[i/col][i%col] == 1 {
                res += 1;
                if grid[i/col].get(i%col + 1) != None && grid[i/col][i%col+1] == 1 {
                    edge += 1;
                }
                if grid.get(i/col + 1) != None && grid[i/col+1][i%col] == 1 {
                    edge += 1;
                }
            }
        }
        res*4 - edge*2                
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
12 ms
, 在所有 rust 提交中击败了
94.12%
的用户
内存消耗 :
2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
