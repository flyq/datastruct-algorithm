fn main() {
    println!("Hello, world!");
    let a = vec![7,1,5,3,6,4];
    println!("{}", Solution::max_profit(a));
}
pub struct Solution {
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut maxprofit = 0;
        let mut minprice = 1000_000_000;
        
        for i in 0..prices.len() {
            if prices[i] < minprice {
                minprice = prices[i];
            } else if prices[i] - minprice > maxprofit {
                maxprofit = prices[i] - minprice;
            }
        }
        maxprofit            
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
