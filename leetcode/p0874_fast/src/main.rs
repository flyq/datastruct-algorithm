fn main() {
    println!("Hello, world!");
}

pub struct Solution{}

use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let ob_set = obstacles
            .iter()
            .map(|v| (v[0], v[1]))
            .collect::<HashSet<_>>();
        let mut x = 0;
        let mut y = 0;
        let mut dir = (0, 1);
        let mut max_dis = 0;

        for com in commands {
            if com == -1 {
                dir = (dir.1, -dir.0);
            } else if com == -2 {
                dir = (-dir.1, dir.0);
            } else {
                for _ in 0..com {
                    if !ob_set.contains(&(x + dir.0, y + dir.1)) {
                        x += dir.0;
                        y += dir.1;
                    }
                }
                max_dis = max_dis.max(x * x + y * y);
            }
        }

        max_dis
    }
}


/*
执行结果：
通过
显示详情
执行用时 :
20 ms
, 在所有 rust 提交中击败了
50.00%
的用户
内存消耗 :
3 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
