fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut res = 0;
        'out: for i in words {
            let mut chars1:Vec<char> = chars.chars().collect();
            let temps: Vec<char> = i.chars().collect();
            for j in 0..temps.len() {
                if chars1.iter().find(|&&x| x == temps[j]) == None {
                    continue 'out;
                } else {
                    if  j == temps.len()-1 {
                        res += temps.len();
                    }
                    chars1.remove(chars1.iter().position(|&x| x == temps[j]).unwrap());
                }
                
            }
        }
        res as i32
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
20 ms
, 在所有 rust 提交中击败了
33.33%
的用户
内存消耗 :
2.2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/

/*
执行结果：
通过
显示详情
执行用时 :
32 ms
, 在所有 Rust 提交中击败了
100.00%
的用户
内存消耗 :
2.3 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
