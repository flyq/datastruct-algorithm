fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack1: VecDeque<i32> = VecDeque::new();
        let mut map = HashMap::new();
        let mut res = vec![];

        for i in 0..nums2.len() {
            while stack1.back() != None && stack1.back().unwrap() < &nums2[i] {
                map.insert(stack1.pop_back().unwrap(), nums2[i]);
            }
            stack1.push_back(nums2[i]);
        }
        while stack1.len() > 0 {
            let temp = stack1.pop_back().unwrap();
            map.insert(temp, -1);
        }

        for i in nums1 {
            let temp = map.get(&i);
            res.push(*temp.unwrap());
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
