fn main() {
    println!("Hello, world!");
    
}

pub struct Solution;

impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut left = 0usize;
        let mut right = a.len()-1;
        let mut a = a;

        while left<right {
            if a[left]%2 != 0 && a[right]%2 == 0{
                let temp = a[left];
                a[left] = a[right];
                a[right] = temp;
                left += 1;
                right -= 1;
            } else if a[left]%2 != 0 && a[right]%2 != 0 {
                right -= 1;
            } else if a[left]%2 == 0 && a[right]%2 == 0 {
                left +=1;
            } else {
                left += 1;
                right -= 1;
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
4 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
