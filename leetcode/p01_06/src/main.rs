fn main() {
    println!("Hello, world!");
}

pub struct Solution {}

impl Solution {
    pub fn compress_string(s: String) -> String {
        let len = s.len();
        if len < 2 {
            return s;
        }
        let vs: Vec<char> = s.chars().collect();
        let mut res = String::new();
        let mut temp = 1;
        res.push(vs[0]);
        for i in 1..len {
            if vs[i] == vs[i-1] {
                temp += 1;
            } else {
                res.push_str(&temp.to_string());
                res.push(vs[i]);
                temp = 1;
            }
        }
        res.push_str(&temp.to_string());
        if res.len() >= len {
            return s;
        }
        res        
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
2.4 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
