fn main() {
    println!("Hello, world!");
}
pub struct Solution{}
pub struct Solution1{}

use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let a: HashSet<_> = nums1.iter().collect();
        let b: HashSet<_> = nums2.iter().collect();
        let res: Vec<i32> = a.intersection(&b).map(|v| **v).collect();

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

