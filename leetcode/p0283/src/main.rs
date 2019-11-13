fn main() {
    println!("Hello, world!");
    let mut a = vec![0,1,0,3,12];
    Solution::move_zeroes(&mut a);
    println!("{:?}", a);

}

struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut j = 0_usize;
        for i in 0..len {
            if nums[i] != 0 {
                nums[j] = nums[i];
                j += 1;
            }
        }
        for i in j..len {
            nums[i] = 0;
        }
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
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
