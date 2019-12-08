fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let lens = s.len();
        let lent = t.len();
        if lens == 0 {
            return true;
        }
        if lent == 0 {
            return false;
        }
        let vs: Vec<char> = s.chars().collect();
        let vt: Vec<char> = t.chars().collect();
        
        let mut i = 0;
        let mut j = 0;
        while j < lent {
            if vs[i] == vt[j] {
                i += 1;
                if i > lens - 1 {
                    break;
                }
            }
            j += 1;
        }
        if j == lent && i < lens {
            return false;
        }
        true
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
4 ms
, 在所有 rust 提交中击败了
84.62%
的用户
内存消耗 :
6.2 MB
, 在所有 rust 提交中击败了
33.33%
的用户
*/
