fn main() {
    println!("Hello, world!");
    let a = String::from("ballon");
    println!("{}",Solution::max_number_of_balloons(a));
}
pub struct Solution{}

use std::collections::HashMap;
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let text: Vec<char> = text.chars().collect();
        let mut map = HashMap::new();
        for i in 0..text.len() {
            let count = map.entry(text[i]).or_insert(0);
            *count += 1;
        }
        let mut min = text.len();
        let mut minc = ' ';
        for i in "balon".to_string().chars() {
            if map.get(&i) == None {
                return 0;
            } else if map[&i] < min {
                min = map[&i];
                minc = i;
            }
        }
        if minc == 'l' || minc == 'o' {
            return (min/2) as i32;
        } else if map[&'l'] < 2*min && map[&'l'] <= map[&'o'] {
            return (map[&'l']/2) as i32;
        } else if map[&'o'] < 2*min && map[&'o'] <= map[&'l'] {
            return (map[&'o']/2) as i32;
        }
        min as i32           
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
