use std::cmp::max;

fn main() {
    println!("Hello, world!");
}

pub struct Solution{}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.clone() + &str2 != str2.clone() + &str1 {
            return "".to_string();
        }

        let lmax = max(str1.len(), str2.len());
        let lmin = str1.len() + str2.len() - lmax;
        let gcd_num = Self::gcd(lmax, lmin);

        return (&str1[..gcd_num]).to_string();        
    }

    pub fn gcd(mut max: usize, mut min: usize) -> usize {
        while min != 0 {
            let t = min;
            min = max % min;
            max = t;
        }
        return max;
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
2.1 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
