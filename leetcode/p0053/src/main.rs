fn main() {
    println!("Hello, world!");
    let a = vec![-2,1,-3,4,-1,2,1,-5,4];
    println!("{}", Solution::max_sub_array(a));
}

struct Solution{}


impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut next = 0;
        for i in 0..nums.len() {
            if next + nums[i] <= 0 {
                next = 0;
            } else {
                next += nums[i];
            }
            if next > max {
                max = next;
            }
        }
        if max == 0 {
            max = nums[0];
            for i in nums {
                if i > max {
                    max = i;
                }
            }
        }
        max
    }
}

/*
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
