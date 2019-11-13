fn main() {
    println!("Hello, world!");
    let a = vec![1,2,3,1];
    println!("{}", Solution::contains_nearby_duplicate(a, 3))
}

struct Solution{}
struct Solution1{}

use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut a = HashMap::new();
        for i in 0..nums.len() {
            let temp = a.get(&nums[i]);
            if temp != None && &i-temp.unwrap() <= k as usize {
                return true;
            } else if temp != None && &i-temp.unwrap() > k as usize {
                let index = a.entry(nums[i]).or_insert(0);
                *index = i;
            } else {
                a.insert(nums[i], i);
            }
        }
        false 
    }
}

    
/*
执行结果：
通过
显示详情
执行用时 :
8 ms
, 在所有 rust 提交中击败了
73.68%
的用户
内存消耗 :
5.2 MB
, 在所有 rust 提交中击败了
25.00%
的用户
*/


impl Solution1 {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut a = HashMap::new();
        for i in 0..nums.len() {
            let temp = a.get(&nums[i]);
            if temp != None && &i-temp.unwrap() <= k as usize {
                return true;
            } else {
                a.insert(nums[i], i);
            }
        }
        false 
    }
}


/*
执行结果：
通过
显示详情
执行用时 :
8 ms
, 在所有 rust 提交中击败了
73.68%
的用户
内存消耗 :
5.3 MB
, 在所有 rust 提交中击败了
25.00%
的用户
*/
