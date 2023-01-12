fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        nums.into_iter().enumerate().for_each(|(i, a)| {
            if i == 0 {
                res.push(a);
            } else {
                res.push(a + res[i - 1]);
            }
        });
        res
    }
}
