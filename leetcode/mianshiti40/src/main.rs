fn main() {
    println!("Hello, world!");
}

pub struct Solution {}
pub struct Solution1 {}

use std::collections::BinaryHeap;
impl Solution {
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![];
        }
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k);

        for i in 0..arr.len() {
            if heap.len() < k {
                heap.push(arr[i]);
            } else {
                if heap.peek().unwrap() > &arr[i] {
                    heap.pop();
                    heap.push(arr[i]);
                }
            }
        }
        heap.into_vec()
    }
}


/*
执行结果：
通过
显示详情
执行用时 :
16 ms
, 在所有 Rust 提交中击败了
44.44%
的用户
内存消耗 :
2.3 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/

/*
执行结果：
通过
显示详情
执行用时 :
4 ms
, 在所有 Rust 提交中击败了
100.00%
的用户
内存消耗 :
2.1 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
    


impl Solution1 {
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![];
        }
        let k = k as usize;
        let mut heap = BinaryHeap::new();

        for i in 0..arr.len() {
            heap.push(arr[i]);
            if heap.len()> k {
                heap.pop();
            }
        }
        heap.into_vec()
    }
}


/*
执行结果：
通过
显示详情
执行用时 :
8 ms
, 在所有 Rust 提交中击败了
100.00%
的用户
内存消耗 :
2.1 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
