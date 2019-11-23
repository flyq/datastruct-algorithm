fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let c = coordinates;
        let len = c.len();
        if len == 2 {
            return true;
        }
        for i in 2..len {
            if (c[i][1]-c[1][1])*(c[1][0]-c[0][0]) != (c[i][0]-c[1][0])*(c[1][1]-c[0][1]) {
                return false;
            }
        }
        true
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
