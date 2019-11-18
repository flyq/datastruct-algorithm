fn main() {
    println!("Hello, world!");
}
pub struct Solution {}
pub struct Solution1 {}


use std::collections::HashMap;
impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        let mut min = 10001;
        for i in deck {
            let counter = map.entry(i).or_insert(0);
            *counter += 1;
        }
        for key in map.keys() {
            if map[key] < min {
                min = map[key];
            }
        }
        for value in map.values() {
            if min == 1 || value % min != 0 {
                return false;
            }
        }
        true                   
    }
}
/*
执行结果：
解答错误
显示详情
输入:
[1,1,1,1,2,2,2,2,2,2]
输出
false
预期结果
true
 */







impl Solution1 {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        let mut min = 10001;
        for i in deck {
            let counter = map.entry(i).or_insert(0);
            *counter += 1;
        }
        for key in map.keys() {
            if map[key] < min {
                min = map[key];
            }
        }
        for value in map.values() {
            min = Solution1::gcd(*value, min);
            if min == 1 {
                return false;
            }
        }
        true                   
    }
    pub fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        Solution1::gcd(b, a%b)
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
4 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
