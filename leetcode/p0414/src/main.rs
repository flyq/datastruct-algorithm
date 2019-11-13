fn main() {
    println!("Hello, world!");
    let a = vec![2, 2, 3, 1];
    println!("{}", Solution::third_max(a));
}
struct Solution {}

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_by(|a, b| b.cmp(a));
        let mut count = 0;
        for i in 0..nums.len()-1 {
            if  nums[i] != nums[i+1] {
                count += 1;
                if count >= 2 {
                    return nums[i+1];
                } 
            }
        }
        nums[0]
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
