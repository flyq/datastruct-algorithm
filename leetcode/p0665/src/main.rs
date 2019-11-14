fn main() {
    println!("Hello, world!");
}
struct Solution{}
struct Solution1 {}
struct Solution2 {}

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut temp1 = nums.clone();
        
        for i in 0..nums.len() {
            temp1.remove(i);
            let mut temp2 = temp1.clone();
            
            temp2.sort();
            if temp2 == temp1 {
                return true;
            }
            temp1 = nums.clone();
        }
        false 
    }
}

/*
执行结果：
超出时间限制
显示详情
最后执行的输入：
[7345,2996,
查看全部
 */


impl Solution1 {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut count = 0;
        for i in 0..nums.len()-1 {
            if nums[i] > nums[i+1] {
                if count > 0 {
                    return false;
                }
                count += 1;
            }
        }
        true
    }
}
/*
执行结果：
解答错误
显示详情
输入:
[3,4,2,3]
输出
true
预期结果
false
*/


impl Solution2 {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut count = 0;
        for i in 0..nums.len()-1 {
            if nums[i] > nums[i+1] {
                if count > 0 {
                    return false;
                }
                if nums.get(i-1) != None && nums.get(i+2) != None {
                    if nums[i-1] > nums[i+1] && nums[i] > nums[i+2] {
                        return false;
                    }
                }
                count += 1;
            }
        }
        true
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
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
 */
