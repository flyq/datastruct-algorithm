fn main() {
    println!("Hello, world!");
    println!("{}", Solution::get_sum(20,30));
        
}
pub struct Solution{}

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut and = a & b;
        let mut xor = a ^ b;

        while and != 0  {
            let temp = xor;
            and <<= 1;
            xor ^= and;
            and &= temp;
        }
        xor            
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
