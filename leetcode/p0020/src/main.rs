fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v: Vec<char> = vec![];
        let vs: Vec<char> = s.chars().collect();
        for i in 0..vs.len() {
            let vlen = v.len();
            if vlen == 0 {
                v.push(vs[i]);
            } else if Solution::is_con(v[vlen-1],vs[i]) {
                v.pop();
            } else {
                v.push(vs[i]);
            }
        }
        if v.len() != 0 {
            return false;
        }
        true                
    }
    
    pub fn is_con(a: char, b: char) -> bool {
        if a=='(' && b==')' || a=='[' && b==']' || a=='{' && b=='}' {
            return true;
        }
        false
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
