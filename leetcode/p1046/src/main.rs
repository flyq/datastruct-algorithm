fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::BinaryHeap;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        for i in 0..stones.len() {
            heap.push(stones[i]);
        }
        while heap.len() > 1 {
            let b1 = heap.pop().unwrap();
            let b2 = heap.pop().unwrap();
            if b1 != b2 {
                heap.push(b1-b2);
            }
        }
        if heap.len() == 0 {
            return 0;
        }
        heap.pop().unwrap()        
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
