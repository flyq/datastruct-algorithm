fn main() {
    println!("Hello, world!");
    let a = vec![1,2,3,4];
    
    println!("{}",Solution1::contains_duplicate(a));
}
struct Solution{}
struct Solution1{}

use std::collections::HashMap;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut a = HashMap::new();
        for i in nums {
            if a.get(&i) != None {
                return true;
            } else {
                a.insert(i, 1);
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
51.85%
的用户
内存消耗 :
4.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/


impl Solution1 {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        if nums.len() == 0 {
            return false;
        }
        let mut nums = nums;
        nums.sort();
        for i in 0..nums.len()-1 {
            if nums[i] == nums[i+1] {
                return true;
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
0 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.7 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
