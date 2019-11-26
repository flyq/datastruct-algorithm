fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::HashMap;
impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut map = HashMap::new();
        let len = points.len();
        for i in 0..len {
            for j in 0..len {
                let dis = (points[i][0]-points[j][0]).pow(2) + (points[i][1]-points[j][1]).pow(2);
                let count = map.entry(dis).or_insert(0);
                res += *count * 2;
                *count += 1;
            }
            map.clear();
        }
        res
    }
}

/*
这么垃圾的算法都能 100%。。。就是两个循环，O(N^2) 的复杂度
执行结果：
通过
显示详情
执行用时 :
80 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户

*/
