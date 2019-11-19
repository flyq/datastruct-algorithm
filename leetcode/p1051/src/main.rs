fn main() {
    println!("Hello, world!");
}

pub struct Solution{}

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut a = heights.clone();
        a.sort();
        for i in 0..a.len() {
            if a[i] != heights[i] {
                res += 1;
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
0 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
1.9 MB
, 在所有 rust 提交中击败了
100.00%
的用户
 */

