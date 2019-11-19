fn main() {
    println!("Hello, world!");
    let a1 = String::from("bella");
    let a2 = String::from("label");
    let a3 = String::from("roller");
    let a = vec![a1, a2, a3];
    println!("{:?}",Solution::common_chars(a));
}
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let mut b: Vec<HashMap<char,i32>> = vec![];
        let mut res: Vec<String> = vec![];
        for i in &a {
            let mut temp = HashMap::new();
            let char_vec:Vec<char> = i.chars().collect();
            for c in char_vec {
                let count = temp.entry(c).or_insert(0);
                *count += 1;
            }
            b.push(temp);
        }
        for k in b[0].keys() {
            let mut min = b[0][k];
            for i in 1..a.len() {
                if b[i].get(k) == None {
                    min = 0;
                } else if b[i][k] < min {
                    min = b[i][k];
                }
            }
            while min > 0 {
                res.push(k.to_string());
                min -= 1;
            }
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
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
