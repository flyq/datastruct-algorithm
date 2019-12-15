fn main() {
    println!("Hello, world!");
}
pub struct Solution{}


use std::collections::BinaryHeap;

impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut mincost = BinaryHeap::with_capacity(costs.len()/2);
        for i in 0..costs.len() {
            res += costs[i][0];
            let delta = costs[i][1]-costs[i][0];
            if mincost.len() < mincost.capacity() {
                mincost.push(delta);
            } else {
                if mincost.peek().unwrap() > &delta {
                    mincost.pop();
                    mincost.push(delta);
                }
            }
        }
        while mincost.len() > 0 {
            res += mincost.pop().unwrap();
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
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
