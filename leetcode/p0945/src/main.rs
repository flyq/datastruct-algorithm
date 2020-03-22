fn main() {
    println!("Hello, world!");
}

pub struct Solution {}

impl Solution {
    pub fn min_increment_for_unique(a: Vec<i32>) -> i32 {
        let mut a = a;
        a.sort();
        let mut res = 0;
        for i in 1..a.len() {
            if a[i] <= a[i-1] {
                res = a[i-1]+1-a[i];
                a[i] = a[i-1]+1;
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
20 ms
, 在所有 Rust 提交中击败了
100.00%
的用户
内存消耗 :
2.5 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
