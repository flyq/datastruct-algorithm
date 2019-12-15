fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

impl Solution {
    pub fn largest_sum_after_k_negations(a: Vec<i32>, k: i32) -> i32 {
        let mut a = a;
        a.sort();
        let mut small_zero = 0;
        for i in 0..a.len() {
            if a[i] < 0 {
                small_zero += 1;
            }
        }
        if k < small_zero {
            for i in 0..k as usize {
                a[i] = -a[i];
            }
            return a.iter().sum();
        }
        let mut i = 0;
        while a[i] < 0 && i < a.len() {
            a[i] = -a[i];
            i += 1;
        }
        if (k as usize-i)%2 == 0 {
            return a.iter().sum();
        } else {
            if a.get(i-1) == None || a[i].abs() < a[i-1].abs() {
                a[i] = -a[i];
            } else {
                a[i-1] = -a[i-1];
            }
        }
        a.iter().sum()        
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
