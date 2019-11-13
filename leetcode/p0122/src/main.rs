fn main() {
    println!("Hello, world!");
    let a = vec![7,1,5,3,6,4];
    println!("{}", Solution::max_profit(a));
}

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let mut max_profit = 0;
        let mut k = -1;
        
        for i in 0..len {
            if  k == -1 {
                if i != len-1 && prices[i+1]>prices[i] {
                    k = prices[i];
                }
            } else {
                if i == len-1 || prices[i+1]<prices[i] {
                    max_profit += prices[i] - k;
                    k = -1;
                }
            }
        }
        max_profit                
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
2.2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
