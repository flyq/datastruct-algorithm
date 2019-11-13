fn main() {
    println!("Hello, world!");
    let a = vec![4,3,2,7,8,2,3,1];
    println!("{:?}", Solution1::find_disappeared_numbers(a));
}

struct Solution{}
struct Solution1{}

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = (1..nums.len() as i32+1).collect();
        for i in nums {
            res.retain(|&x| x != i);
        }
        res
    }
}

/*
执行结果：
超出时间限制
显示详情
最后执行的输入：
[38113,47352,47293,25351,41449,
查看全部
 */

impl Solution1 {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut nums = nums;
        for i in 0..nums.len() {
            let index = nums[i].abs() as usize -1;
            nums[index] = -nums[index].abs();
        }
        for i in 0..nums.len() {
            if nums[i] > 0 {
                res.push((i+1)as i32);
            }
        }
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
86.67%
的用户
内存消耗 :
2.6 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
