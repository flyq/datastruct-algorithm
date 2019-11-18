fn main() {
    println!("Hello, world!");
}

pub struct Solution{}

impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        let len = a.len();
        if len < 3 {
            return false;
        }
        let mut max = 0;
        let mut maxindex = 0;
        
        for i in 0..len {
            if a[i] > max {
                max = a[i];
                maxindex = i;
            }
        }
        if a[0] == max || a[len-1] == max {
            return false;
        }
        for i in 1..maxindex+1 {
            if a[i] <= a[i-1] {
                return false;
            }
        }
        for i in maxindex+1..len {
            if a[i] >= a[i-1] {
                return false;
            }
        }
        true
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
