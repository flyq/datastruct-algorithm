fn main() {
    println!("Hello, world!");
}

pub struct Solution{}

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        if bills.len() == 0 {
            return true;
        }

        let mut count_5 = 0;
        let mut count_10 = 0;
        for i in 0..bills.len() {
            if bills[i] == 5 {
                count_5 += 1;
            } else if bills[i] == 10 {
                if count_5 == 0 {
                    return false;
                }
                count_5 -= 1;
                count_10 += 1;
            } else {
                if count_5 == 0 || (count_5 < 3 && count_10 == 0) {
                    return false;
                }
                if count_10 == 0 {
                    count_5 -= 3;
                } else {
                    count_5 -= 1;
                    count_10 -= 1;
                }
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
