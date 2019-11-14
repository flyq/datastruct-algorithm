fn main() {
    println!("Hello, world!");
}
struct Solution{}

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut temp = Vec::new();
        let mut v = nums.clone();
        v.sort();
        for i in 0..nums.len() {
            if nums[i] != v[i] {
                temp.push(i);
            }
        }
        let len = temp.len();
        if len == 0 {
            return 0;
        }
        (temp[len-1]-temp[0] + 1) as i32
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
8 ms
, 在所有 rust 提交中击败了
50.00%
的用户
内存消耗 :
2.3 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
