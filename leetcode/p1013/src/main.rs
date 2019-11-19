fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        if sum % 3 != 0 {
            return false;
        }
        sum /= 3;
        let mut sum1 = 0;
        let mut i = 0;
        while sum1 != sum && i < a.len() {
            sum1 += a[i];
            i += 1;
        }
        if i > a.len()-2 {
            return false;
        }
        sum1 = 0;
        while sum1 != sum && i < a.len() {
            sum1 += a[i];
            i += 1;
        }
        if i > a.len() -1 {
            return false;
        }
        true                
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
2.6 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
