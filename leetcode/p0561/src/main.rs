fn main() {
    println!("Hello, world!");
    let a = vec![1,4,3,2];
    println!("{}", Solution::array_pair_sum(a));
}

struct Solution {}

impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut sum = 0;
        nums.sort();
        for i in (0..nums.len()).filter(|x| x%2==0) {
            sum += nums[i];
        }
        sum
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
12 ms
, 在所有 rust 提交中击败了
92.31%
的用户
内存消耗 :
2.3 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
