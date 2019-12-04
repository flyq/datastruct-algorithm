fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut time = 0;
        let mut t1;
        let mut t2;
        for i in 1..points.len() {
            t1 = (points[i][0] - points[i-1][0]).abs();
            t2 = (points[i][1] - points[i-1][1]).abs();
            if t1>t2 {
                time += t1;
            } else {
                time += t2;
            }
        }
        time
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
