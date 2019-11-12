fn main() {
    println!("Hello, world!");
    println!("{:?}", Solution::generate(5));
}

pub struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut a = vec![vec![]; num_rows as usize];
        
        for i in 0..num_rows as usize {
            for j in 0..i+1 {
                if j == 0 || j == i {
                    a[i].push(1);
                } else {
                    let x = a[i-1][j-1].clone();
                    let y = a[i-1][j].clone();
                        
                    a[i].push(x + y);
                }
            }
        }
        a
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
