fn main() {
    println!("Hello, world!");
}
struct Solution {}

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut index = 0;
        for i in 0..nums.len() {
            if nums[i] > max {
                max = nums[i];
                index = i;
            }
        }

        for i in 0..nums.len() {
            if i != index && nums[i]*2 > max {
                return -1;
            }
        }
        index as i32
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
