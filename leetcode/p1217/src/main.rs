fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

impl Solution {
    pub fn min_cost_to_move_chips(chips: Vec<i32>) -> i32 {
        let mut ou = 0;
        let len = chips.len();
        for i in 0..len {
            if chips[i] % 2 == 0 {
                ou += 1;
            }
        }
        if ou > len-ou {
            return (len-ou) as i32;
        }
        ou as i32
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
