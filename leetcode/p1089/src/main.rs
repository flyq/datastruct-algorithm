fn main() {
    println!("Hello, world!");
    let mut a = vec![1, 0, 2, 3, 0, 4, 5, 0];
    Solution::duplicate_zeros(&mut a);
    println!("{:?}", a);
}
pub struct Solution {}

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut amount0 = 0usize;
        let len = arr.len();
        for i in 0..len {
            if arr[i] == 0 {
                amount0 += 1;
            }
        }
        for _i in 0..amount0 {
            arr.push(0);
        }
        for i in (0..len).rev() {
            if arr[i] != 0 {
                arr[i + amount0] = arr[i];
            } else {
                arr[i + amount0 - 1] = arr[i];
                arr[i + amount0] = 0;
                amount0 -= 1;
            }
        }
        arr.truncate(len);
    }
}

/*
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut temp = vec![];
        for i in arr.iter() {
            temp.push(*i);
            if *i == 0 {
                temp.push(*i);
            }
        }
        *arr = temp[..arr.len()].to_vec();
    }
}
*/

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
