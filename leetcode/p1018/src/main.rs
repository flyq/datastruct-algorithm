fn main() {
    println!("Hello, world!");
    let a = vec![0,1,1,1];
    Solution::prefixes_div_by5(a);
}
pub struct Solution{}

impl Solution {
    pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        let mut temp = 0;
        let mut b = vec![];
        for i in 0..a.len() {
            temp = ((temp << 1) + a[i])%5;
            if temp == 0 {
                b.push(true);
            } else {
                b.push(false);
            }
        }
        b        
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
2.4 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
