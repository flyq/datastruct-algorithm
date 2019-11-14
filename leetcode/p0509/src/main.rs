fn main() {
    println!("Hello, world!");
}

struct Solution{}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut pre = 0;
        let mut next = 1;
        let mut temp = 0;
        for i in 1..n {
            temp = next;
            next += pre;
            pre = temp;
        }
        next
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
1.9 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
