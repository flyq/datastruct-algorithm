fn main() {
    println!("Hello, world!");
}

pub struct Solution {
}

impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut a = a;
        let collen = a[0].len();
        let rowlen = a.len();
        for j in 0..collen/2 {
            for i in 0..rowlen {
                let temp = a[i][j];
                a[i][j] = a[i][collen-1-j];
                a[i][collen-1-j] = temp;
            }
        }

        for i in 0..rowlen {
            for j in 0..collen {
                a[i][j] ^= 1;
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
