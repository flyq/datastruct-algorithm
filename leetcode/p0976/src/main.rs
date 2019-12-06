fn main() {
    println!("Hello, world!");
}
pub struct Solution{}


impl Solution {
    pub fn largest_perimeter(a: Vec<i32>) -> i32 {
        let mut a = a;
        let mut i = a.len();
        let mut res = 0;
        a.sort();
        loop {
            if i >= 3 && a[i-1] < a[i-2]+a[i-3] {
                res = a[i-1]+a[i-2]+a[i-3];
                break;
            } else if i < 3 {
                break;
            }
            i -= 1;
        }
        res            
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
8 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
