fn main() {
    println!("Hello, world!");
    let a = vec![9,6,4,2,3,5,7,0,1];
    println!("{}", Solution::missing_number(a));
}

pub struct Solution {}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        let sum:i32 = nums.iter().sum();
        (1+len)*len/2 - sum
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
2.2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
