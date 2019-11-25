fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::HashMap;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map1 = HashMap::new();
        let mut res = vec![];
        for i in 0..nums1.len() {
            let count = map1.entry(nums1[i]).or_insert(0);
            *count += 1;
        }
        for i in 0..nums2.len() {
            if map1.get(&nums2[i]) != None {
                res.push(nums2[i]);
                let count = map1.entry(nums2[i]).or_insert(0);
                *count -= 1;
                if *count == 0 {
                    map1.remove(&nums2[i]);
                }
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
