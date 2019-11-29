fn main() {
    println!("Hello, world!");
}
pub struct Solution {
}

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let v: Vec<String> = text.split(' ').map(|s| s.to_string()).collect();
        let mut res:Vec<String> = vec![];
        for i in 0..v.len()-2 {
            if v[i] == first && v[i+1] == second {
                res.push(v[i+2].clone());
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
0 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
