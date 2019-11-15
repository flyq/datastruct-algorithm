fn main() {
    println!("Hello, world!");
    let a = vec![1,7,3,6,5,6];
    println!("{}", Solution::pivot_index(a));
}

// bug !!!
pub struct Solution{}
pub struct Solution1{}
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return -1;
        }
        let mut low = 0;
        let mut big = nums.len()-1;
        let mut sum = 0;
        while low < big {            
            if sum < 0 {
                if  -nums[big] < nums[low] {
                    sum += nums[low];
                    low += 1;
                } else {
                    sum -= nums[big];
                    big -= 1;
                }
            } else if sum > 0 {
                if -nums[big] > nums[low] {
                    sum += nums[low];
                    low += 1;
                } else {
                    sum -= nums[big];
                    big -= 1;
                }
            } else {
                sum += nums[low];
                low += 1;
            }
        }
        if sum != 0 {
            return -1;
        }
        while low > 0 && nums[low] == 0 && nums[low-1] == 0 {
            low -= 1;
        }
        low as i32
    }
}

/*
执行结果：
解答错误
显示详情
输入:
[-1,-1,-1,0,-1,-1]
输出
-1
预期结果
2

当左右效果相等时，失效
 */


impl Solution1 {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut left = 0;
        for i in &nums {
            sum += i;
        }
        for i in 0..nums.len() {
            if left*2 == sum - nums[i] {
                return i as i32;
            } else {
                left += nums[i];
            }
        }
        -1            
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
