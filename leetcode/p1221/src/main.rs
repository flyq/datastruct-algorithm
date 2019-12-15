fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut sv: Vec<char> = vec![];
        let mut res = 0;
        for i in s.chars() {
            let sv_len = sv.len();
            if sv_len == 0 {
                sv.push(i);
                res += 1;
            } else if (sv[sv_len-1] == 'R' && i == 'L') || (sv[sv_len-1] == 'L' && i == 'R') {
                sv.pop();
            } else {
                sv.push(i);
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
2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
