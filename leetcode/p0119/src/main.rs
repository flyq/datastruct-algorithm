fn main() {
    println!("Hello, world!");
    println!("{:?}", Solution::get_row(33));
}

pub struct Solution {
}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut a = vec![];
        let index = row_index as i64;

        for i in 0..index + 1 {
            if i == 0 {
                a.push(1i32);
            } else {
                let temp = a[(i-1) as usize] as i64;
                a.push((temp*(row_index as i64 +1-i)/i) as i32);
            }
        }
        a
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
