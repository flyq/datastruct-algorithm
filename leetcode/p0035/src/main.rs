fn main() {
    println!("Hello, world!");
    let a = vec![1,3,5,6];
    println!("{:}", Solution::search_insert(a, 0));
}
struct Solution {}
     
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut max = nums.len() - 1;
        let mut min = 0;
        let mut temp = (max+min)/2;
        if nums[min] >= target {
            return min as i32;
        }
        if nums[max] < target {
            return (max+1) as i32;
        }
        while max-min > 1 {
            if nums[temp] == target {
                return temp as i32;
            } else if nums[temp] > target {
                max = temp;
                temp = (max+min)/2;
            } else {
                min = temp;
                temp = (max+min)/2;
            }
        }
        max as i32
    }
}
