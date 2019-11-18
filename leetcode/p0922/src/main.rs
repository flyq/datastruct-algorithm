fn main() {
    println!("Hello, world!");
}
pub struct Solution{}


impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        let mut index1 = 0;
        let mut index2 = 1;
        while index1 < a.len() && index2 < a.len() {
            if a[index1]%2 != 0 && a[index2]%2 == 0 {
                let temp = a[index1];
                a[index1] = a[index2];
                a[index2] = temp;
                index1 += 2;
                index2 += 2;
            } else if a[index1]%2 == 0 && a[index2]%2 == 0 {
                index1 += 2;
            } else if a[index1]%2 != 0 && a[index2]%2 != 0 {
                index2 += 2;
            } else {
                index1 += 2;
                index2 += 2;
            }
        }
        a            
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
20 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
