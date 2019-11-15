fn main() {
    println!("Hello, world!");
}

struct Solution{}

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut max = 1;
        let mut now = 1;
        if nums.len() == 0 {
            return 0;
        }
        for i in 0..nums.len()-1 {
            if nums[i+1] > nums[i] {
                now += 1;
            } else {
                now = 1;
            }
            if now > max {
                max = now;
            }
        }
        max
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
