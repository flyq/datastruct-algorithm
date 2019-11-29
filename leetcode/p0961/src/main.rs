fn main() {
    println!("Hello, world!");
}
pub struct Solution{}
use std::collections::HashMap;
impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for i in 0..a.len() {
            if map.get(&a[i]) == None {
                map.insert(a[i],1);
            } else {
                return a[i];
            }
        }
        0
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
4 ms
, 在所有 rust 提交中击败了
66.67%
的用户
内存消耗 :
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
