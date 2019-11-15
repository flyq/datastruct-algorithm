fn main() {
    println!("Hello, world!");
}
struct Solution {}

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let rowlen = matrix.len();
        let collen = matrix[0].len();
        if rowlen == 1 || collen == 1 {
            return true;
        }
        
        for i in 0..rowlen-1 {
            for j in 0..collen-1 {
                if matrix[i][j] != matrix[i+1][j+1] {
                    return false;
                }
            }
        }
        true
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
