fn main() {
    println!("Hello, world!");
    let a = vec![vec![1,1],vec![2,2],vec![1,1],vec![1,2],vec![1,2],vec![1,1]];
    println!("{}",Solution::num_equiv_domino_pairs(a));
}
pub struct Solution{}

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut d = dominoes;
        let mut res = 0;
        let mut amount = 1;
        let mut i = 1;
        while d.len() > 1 {
            while i<d.len() {
                if (d[i][0] == d[0][0] && d[i][1] == d[0][1]) || (d[i][0] == d[0][1] && d[i][1] == d[0][0]) {
                    amount += 1;
                    d.remove(i);
                    println!("{} {:?}",amount, d);
                } else {
                    i = i + 1;
                }                
            }            
            res += amount * (amount-1) / 2;
            d.remove(0);
            i = 1;
            amount = 1;
            println!("{} {:?}",amount, d);
        }
        res        
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
196 ms
, 在所有 rust 提交中击败了
33.33%
的用户
内存消耗 :
4.4 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
