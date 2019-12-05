fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::VecDeque;
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut res: VecDeque<char> = VecDeque::new();
        for i in s.chars() {
            if res.back() != None && res.back().unwrap() == &i {
                res.pop_back();
            } else {
                res.push_back(i);
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
