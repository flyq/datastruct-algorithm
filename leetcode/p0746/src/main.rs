fn main() {
    println!("Hello, world!");
}
struct Solution {}
struct Solution1 {}

// bug!!!
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut len = cost.len();
        let mut i = 0;
        let mut min = 0;
        let mut cost = cost;

        while i < len  {
            if  cost[i] <= cost[i+1] {
                min += cost[i];
                if i >= len - 2 {
                    return min;
                }
                i += 1;
            } else {
                min += cost[i+1];
                if i+1 >= len - 2 {
                    return min;
                }
                i += 2;
            }
        }
        min
    }
}

/*
执行结果：
解答错误
显示详情
输入:
[0,1,2,2]
输出
3
预期结果
2
 */

use std::cmp;
impl Solution1 {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut right = 0;
        let mut left = 0;
        for i in 2..cost.len()+1 {
            let mut temp = right;
            right = cmp::min(left + cost[i-2], right + cost[i-1]);
            left = temp;
        }
        right
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
