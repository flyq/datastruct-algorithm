fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if nums.iter().sum::<i32>() == len as i32 {
            return len as i32;
        }
        let mut max = 0;
        for i in 0..len {
            if nums[i] == 0 {
                max = max.max(get(i, &nums));
            }
        }
        max + 1
    }
}

fn get(i: usize, nums: &Vec<i32>) -> i32 {
    let mut j = i as i32 - 1;
    let mut k = i + 1;
    let mut amount1 = 0;
    let mut amount2 = 0;
    while j >= 0 {
        if nums[j as usize] == 1 {
            amount1 += 1;
        } else {
            break;
        }
        j -= 1;
    }
    while k < nums.len() {
        if nums[k] == 1 {
            amount2 += 1;
        } else {
            break;
        }
        k += 1;
    }
    amount1 + amount2
}
