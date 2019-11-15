fn main() {
    println!("Hello, world!");
}
struct Solution {}
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut bits = bits;
      
        while bits.len() > 1 {
            if bits[0] == 1 {
                bits.remove(0);
                bits.remove(0);
            } else if bits[0] == 0 {
                bits.remove(0);
            }
        }
        if bits.len() == 1 {
            return true;
        }
        false
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
