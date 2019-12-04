fn main() {
    println!("Hello, world!");
    println!("{:?}", Solution::powerful_integers(2,1,10));
}
pub struct Solution{}

use std::collections::HashSet;
impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        if bound < 2 {
            return vec![];
        }
        let mut set = HashSet::new();
        let mut temp = 2;
        let mut i = 0;
        let mut j = 0;
        while temp <= bound {
            while temp <= bound {
                set.insert(temp);
                temp = x.pow(i) + y.pow(j);
                j += 1;
                if y == 1 && j > 1 {
                    break;
                }
            }
            i += 1;
            j = 0;
            temp = x.pow(i) + y.pow(j);
            if x == 1 && i > 1 {
                break;
            }
        }
        let res = set.iter().map(|x| *x).collect();
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
2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
