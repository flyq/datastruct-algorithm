fn main() {
    println!("Hello, world!");
    let a = vec![4,3,2,1];
    println!("{:?}", Solution::plus_one(a));
    
}
struct Solution{}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let len = digits.len();
        let mut temp = 0;
        let mut v1: Vec<i32> = Vec::new();
        for i in (0..len).rev() {
            if i == len - 1 {
                if digits[i] == 9 {
                    temp = 1;
                    v1.push(0);
                } else {
                    v1.push(digits[i] + 1);
                    temp = 0;
                }
            } else {
                if digits[i] + temp == 10 {
                    v1.push(0);
                    temp = 1;
                } else {
                    v1.push(digits[i] + temp);
                    temp = 0;
                }
            }
        }
        if temp == 1 {
            v1.push(1);
        }
        let mut v2:Vec<i32> = Vec::new();
        for i in (0..v1.len()).rev() {
            v2.push(v1[i]);
        }
        v2
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
