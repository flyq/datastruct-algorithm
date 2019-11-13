fn main() {
    println!("Hello, world!");
    let mut a = vec![1,2,3,4,5,6,7];
    Solution::rotate(&mut a, 3);
    println!("{:?}", a);
}

struct Solution{}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _i in 0..k%nums.len() as i32 {
            Solution::rotate_1(nums);
        }
    }
    pub fn rotate_1(a: &mut Vec<i32>) {
        let len = a.len();
        let temp = a[len-1];
        
        for i in (1..len).rev() {
            a[i] = a[i-1];
        }
        a[0] = temp;
    }
                
}

/*
执行用时 :
232 ms
, 在所有 rust 提交中击败了
11.54%
的用户
内存消耗 :
2.3 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
