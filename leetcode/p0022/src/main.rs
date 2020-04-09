fn main() {
    println!("Hello, world!");
}

pub struct Solution {} 

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 0 {
            return vec!["".to_string()];
        }
        let mut ans = vec![];
        for i in 0..=n - 1 {
            for left in Solution::generate_parenthesis(n - 1 - i) {
                for right in Solution::generate_parenthesis(i) {
                    ans.push(format!("({}){}", left, right));
                }
            }
        }
        ans
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
0 ms
, 在所有 Rust 提交中击败了
100.00%
的用户
内存消耗 :
2.2 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
