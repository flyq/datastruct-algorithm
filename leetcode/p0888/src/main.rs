fn main() {
    println!("Hello, world!");
    let a = vec![1,2,5];
    let b = vec![2,4];
    println!("{:?}", Solution::fair_candy_swap(a,b));
}
pub struct Solution {}

impl Solution {
    pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut suma = 0;
        let mut sumb = 0;
        for i in &a {
            suma += i;
        }
        for i in &b {
            sumb += i;
        }

        let delta = (suma + sumb)/2 - suma;
        for i in &a {
            for j in &b {
                if *i + delta == *j {
                    return vec![*i,*j];
                }
            }
        }
        vec![]
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
232 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
