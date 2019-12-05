fn main() {
    println!("Hello, world!");
    let a = "(()())(())(()(()))".to_string();
    println!("{:?}", Solution::remove_outer_parentheses(a));
}
pub struct Solution{}

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut left = 1;
        let mut start = 0;
        let mut count = 0;
        let mut res: Vec<char> = vec!['('];
        let chars: Vec<char> = s.chars().collect();
        for i in 1..chars.len() {
            res.push(chars[i]);
            
            if chars[i] == '(' {
                left += 1;
            } else {
                left -= 1;
            }
            if left == 0 {
                res.remove(start);
                res.pop();
                start = i - 2*count - 1;
                count += 1;
            }
        }
        res.into_iter().collect()
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
