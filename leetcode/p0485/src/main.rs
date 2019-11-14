fn main() {
    println!("Hello, world!");
    
}

struct Solution {
}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut max = vec![0;len];
        let mut j = 0;
        for i in 0..len {
            if nums[i] == 1 {
                max[j] += 1;
            } else {
                j += 1;
            }
        }
        let mut temp = 0;
        for i in max {
            if i > temp {
                temp = i;
            }
        }
        temp
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
4 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.3 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
