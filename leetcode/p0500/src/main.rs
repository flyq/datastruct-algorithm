fn main() {
    println!("Hello, world!");
}

pub struct Solution{}

use std::collections::HashMap;
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut map = HashMap::new();
        let mut res = vec![];
        let temp: Vec<char> = "QWERTYUIOPqwertyuiop".to_string().chars().collect();
        for i in 0..temp.len() {
            map.insert(temp[i],1);
        }
        let temp: Vec<char> = "ASDFGHJKLasdfghjkl".to_string().chars().collect();
        for i in 0..temp.len() {
            map.insert(temp[i],2);
        }
        let temp: Vec<char> = "ZXCVBNMzxcvbnm".to_string().chars().collect();
        for i in 0..temp.len() {
            map.insert(temp[i],3);
        }

        'outer: for i in 0..words.len() {
            let temp: Vec<char> = words[i].chars().collect();
            let num = map[&temp[0]];
            for i in 0..temp.len() {
                if map[&temp[i]] != num {
                    continue 'outer;
                }
            }
            res.push(words[i].clone());
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
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/

    
