fn main() {
    println!("Hello, world!");
    let a = vec![vec![1,2,3],vec![4,5,6], vec![7,8,9]];
    println!("{:?}", Solution::transpose(a));
}
pub struct Solution{}

impl Solution {
    pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_amount = a.len();
        let col_amount = a[0].len();
        let mut res = vec![vec![0;row_amount];col_amount];
        
        for i in 0..row_amount*col_amount {
            res[i%col_amount][i/col_amount] = a[i/col_amount][i%col_amount];
        }
        res
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
4 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
