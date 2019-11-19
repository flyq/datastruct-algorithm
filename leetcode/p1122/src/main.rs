fn main() {
    println!("Hello, world!");
    let a = vec![2,3,1,3,2,4,6,7,9,2,19];
    let b = vec![2,1,4,3,9,6];
    println!("{:?}",Solution::relative_sort_array(a,b));
        
}
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..arr1.len() {
            let count = map.entry(arr1[i]).or_insert(0);
            *count += 1;
        }
        let mut res = vec![];
        for i in 0..arr2.len() {
            for _j in 0..map[&arr2[i]] {
                res.push(arr2[i]);
                map.remove(&arr2[i]);
            }
        }
        let mut v = vec![];
        for key in map.keys() {
            v.push(key);
        }
        v.sort();
        for i in v {
            for _j in 0..map[i] {
                res.push(*i);
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
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
