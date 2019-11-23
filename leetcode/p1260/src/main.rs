fn main() {
    println!("Hello, world!");
    let a = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    println!("{:?}", Solution::shift_grid(a, 1));
}
pub struct Solution{}

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut res = grid.clone();
        let n = grid.len();
        let m = grid[0].len();
        let len = m*n;
        let k = k as usize % len;
        if k == 0 {
            return res;
        }
        for i in 0..m*n {
            let j = (i + k) % (m*n);
            res[j/m][j%m] = grid[i/m][i%m];
        }
        res
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
8 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
