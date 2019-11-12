fn main() {
    println!("Hello, world!");
    let mut a = vec![1,2,3,0,0,0];
    let mut b = vec![2,5,6];

    Solution::merge(&mut a, 3, &mut b, 3);
    println!("{:?}", a);
}
pub struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i1 = 0usize;
        let mut i2 = 0usize;
        let mut temp = vec![];
            
        for _i in 0..n+m {
            if i2 >= n as usize || (i1 < m as usize && nums1[i1] < nums2[i2]) {
                temp.push(nums1[i1]);
                i1 += 1;
            } else {
                temp.push(nums2[i2]);
                i2 += 1;
            }
        }
        println!("{:?}", temp);
        
        for i in 0..(n+m) as usize {
            nums1[i] = temp[i];
        }
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
