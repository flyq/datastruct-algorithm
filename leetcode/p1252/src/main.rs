fn main() {
    println!("Hello, world!");
}
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut line = HashMap::new();
        let mut col = HashMap::new();
        for i in 0..indices.len() {
            let linecount = line.entry(indices[i][0]).or_insert(0);
            let colcount = col.entry(indices[i][1]).or_insert(0);
            *linecount += 1;
            *colcount += 1;
        }
        for i in 0..n {
            if line.get(&i) != None && line[&i] % 2 == 0 {
                line.remove(&i);
            }
        }
        
        for i in 0..m {
            if col.get(&i) != None && col[&i] % 2 == 0 {
                col.remove(&i);
            } 
        }

        let l = line.len() as i32;
        let c = col.len() as i32;
        m*l + n*c - 2*l*c
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
