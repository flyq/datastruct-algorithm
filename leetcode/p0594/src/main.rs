fn main() {
    println!("Hello, world!");
    let a = vec![1,3,5,7,9,11,13,15,17];
    println!("{}", Solution::find_lhs(a));
}
pub struct Solution{}


impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let len = nums.len();
        nums.sort();
        if len == 0 || nums[0] == nums[len-1] {
            return 0
        }
        let mut temp1 = 0;
        let mut temp2;
        let mut temp3 = 0;
        let mut max = 0;

        for i in 0..len+1 {
            if i == len {
                let temp3 = i;
                if nums[temp3-1]-nums[temp1] == 1 && temp3-temp1 > max {
                    max = temp3 - temp1;
                }
                continue;
            }
                
            if nums[i] != nums[temp3] {
                temp2 = temp3;
                temp3 = i;
                if nums[temp2]-nums[temp1] == 1 && temp3-temp1 > max {
                    max = temp3 - temp1;
                   
                }
                temp1 = temp2;
            }
        }
        max as i32
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
12 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
