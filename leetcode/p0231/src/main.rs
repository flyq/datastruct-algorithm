fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n == 0 {
            return false;
        } else if n == 1 {
            return true;
        }
        let mut n = n;
        while n != 1 {
            let temp = n>>1;
            if temp<<1 != n {
                return false;
            }
            n = temp;
        }
        true
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
2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
