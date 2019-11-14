fn main() {
    println!("Hello, world!");
    let a = vec![vec![1,2], vec![3,4]];
    println!("{:?}", Solution::matrix_reshape(a,1,4));
}
struct Solution{}

impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let line = nums.len();
        let column = nums[0].len();
        let r = r as usize;
        let c = c as usize;

        if r*c != line*column {
            return nums;
        }
        
        let mut temp = Vec::new();
        let mut res = vec![vec![0;c];r];
        for i in 0..line {
            for j in 0..column {
                temp.push(nums[i][j]);
            }
        }
        for i in 0..r*c {
            res[i/c][i%c]= temp[i];
        }
        res        
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
2.5 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
