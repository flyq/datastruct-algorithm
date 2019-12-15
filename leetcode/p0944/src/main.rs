fn main() {
    println!("Hello, world!");
    let a = vec!["cba".to_string(),"daf".to_string(),"ghi".to_string()];
    println!("{}",Solution::min_deletion_size(a));
}
pub struct Solution{}

impl Solution {
    pub fn min_deletion_size(a: Vec<String>) -> i32 {
        if a.len() == 1 {
            return 0;
        }
        let mut res = 0;
        let mut b: Vec<Vec<char>> = vec![];
        for i in 0..a.len() {
            b.push(a[i].chars().collect());
        }
        for i in 0..b[0].len() {
            for j in 0..b.len()-1 {
                if b[j][i] > b[j+1][i] {
                    res += 1;
                    break;
                }
            }
        }
        res
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
12 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.5 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
