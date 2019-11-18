fn main() {
    println!("Hello, world!");
}

pub struct Solution{}

impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let mut bigger = 0;
        for i in 1..a.len() {
            if  bigger == 0 {
                if a[i] > a[i-1] {
                    bigger = 1;
                } else if a[i] < a[i-1] {
                    bigger = -1;
                }
            } else if bigger == 1 {
                if a[i] < a[i-1] {
                    return false;
                }
            } else if bigger == -1 {
                if a[i] > a[i-1] {
                    return false;
                }
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
8 ms
, 在所有 rust 提交中击败了
66.67%
的用户
内存消耗 :
2.4 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
