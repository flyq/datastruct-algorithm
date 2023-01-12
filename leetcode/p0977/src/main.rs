fn main() {
    println!("Hello, world!");
    let a = vec![-4, -1, 0, 3, 10];
    println!("{:?}", Solution::sorted_squares(a));
}
pub struct Solution {}

impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        a.sort_by_key(|x| x.abs());
        for i in 0..a.len() {
            a[i] = a[i] * a[i];
        }
        a
    }
}

/*
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut res = vec![0; len];
        let mut left = 0;
        let mut right = len - 1;
        for i in (0..len).rev() {
            if nums[left].abs() < nums[right].abs() {
                res[i] = nums[right].pow(2);
                right -= 1;
            } else {
                res[i] = nums[left].pow(2);
                left += 1;
            }
        }
        res
    }
}
*/

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
2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
