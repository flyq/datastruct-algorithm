fn main() {
    println!("Hello, world!");
}
pub struct Solution{}
pub struct Solution2{}

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let mut res = vec![];
        let mut same = 0;
        let mut sum = 0;
        for i in 1..nums.len() {
            if nums[i] == nums[i-1] {
                same = nums[i];
                break;
            }
        }
        for i in 0..nums.len() {
            sum += nums[i];
        }
        let no = (nums.len() as i32+1)*nums.len() as i32/2 - sum + same;
        res.push(same);
        res.push(no);
        res
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
8 ms
, 在所有 rust 提交中击败了
66.67%
的用户
内存消耗 :
2.2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
 */



use std::collections::HashMap;
impl Solution2 {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut same = 0;
        for i in 0..nums.len() {
            if map.get(&nums[i]) == None {
                map.insert(nums[i],1);
            } else {
                same = nums[i];
                break;
            }
        }
        let sum:i32 = nums.iter().sum();
        let no = (nums.len() as i32+1)*nums.len() as i32/2 - sum + same;
        let mut res = vec![];
        res.push(same);
        res.push(no);
        res
    }
}
               
/*
执行结果：
通过
显示详情
执行用时 :
12 ms
, 在所有 rust 提交中击败了
33.33%
的用户
内存消耗 :
2.6 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
