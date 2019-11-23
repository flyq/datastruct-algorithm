fn main() {
    println!("Hello, world!");
}
pub struct Solution{}
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        loop {
            let temp = Solution::fun(n);
            if temp == 1 {
                return true;
            } else if temp == 4 {
                return false;
            }
            n = temp;
        }
        false
    }
    
    pub fn fun(n: i32) -> i32 {
        let mut n = n;
        let mut res = 0;
        while n/10 != 0 {
            res += (n%10).pow(2);
            n /= 10;
        }
        res + n.pow(2)
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
