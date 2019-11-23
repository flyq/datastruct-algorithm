fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut a = arr;
        let len = a.len();
        let mut min = 1000_000_0;
        let mut res = vec![];
            
        a.sort();
        for i in 1..len {
            if a[i]-a[i-1] < min {
                min = a[i]-a[i-1];
            }
        }
        for i in 1..len {
            if a[i]-a[i-1] == min {
                let temp = vec![a[i-1],a[i]];
                res.push(temp);
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
3 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
