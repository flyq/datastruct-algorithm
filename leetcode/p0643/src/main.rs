fn main() {
    println!("Hello, world!");
    let a = vec![1,12,-5,-6,50,3];
    println!("{}", Solution3::find_max_average(a, 4));
}

struct Solution {}
struct Solution1 {}
struct Solution3 {}
    

// bug!!!
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        if k == 1 {
            let mut nums = nums;
            let len = nums.len();
            nums.sort();
            return nums[len-1] as f64;
        }
        let mut sum = 0;
        for i in 0..k {
            sum += nums[i];
        }
        for i in 0..nums.len()-k {
            if nums[i] < nums[i+k] {
                sum = sum + nums[i+k] - nums[i];
            }
        }
        sum as f64 / k as f64
    }
}
/*
执行结果：
解答错误
显示详情
输入:
[4,2,1,3,3]
2
输出
4.5
预期结果
3.0
 */

impl Solution1 {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut sum = 0;
        let mut temp = 0;
        for i in 0..k {
            sum += nums[i];
        }
        println!("{}", sum);
        for i in 0..nums.len()-k+1 {
            for j in i..i+k {
                temp += nums[j];
            }
            if temp > sum {
                sum = temp;
            }
            temp = 0;
            println!("sum {}, temp {}", sum, temp);
        }
        sum as f64 / k as f64
    }
}

/*
执行用时 :
540 ms
, 在所有 rust 提交中击败了
50.00%
的用户
内存消耗 :
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/


