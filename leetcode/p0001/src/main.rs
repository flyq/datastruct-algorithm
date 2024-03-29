fn main() {
    println!("Hello, world!");
    let nums = [2, 7, 11, 15];
    let target = 9;

    let b = Solution::two_sum(nums.to_vec(), target);
    let c = Solution2::two_sum(nums.to_vec(), target);
    let d = Solution3::two_sum(nums.to_vec(), target);
    let f = Solution4::two_sum(nums.to_vec(), target);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
    println!("{:?}", f);
}

pub struct Solution {}
pub struct Solution2 {}
pub struct Solution3 {}
pub struct Solution4 {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    result.push(i as i32);
                    result.push(j as i32);
                }
            }
        }
        result
    }
}

impl Solution2 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        for (key, value) in nums.iter().enumerate() {
            let a = nums
                .iter()
                .enumerate()
                .find(|&(i, x)| x + value == target && i != key);
            if a != None {
                result.push(key as i32);
                result.push(a.unwrap().0 as i32);
                return result;
            }
        }

        result
    }
}

impl Solution3 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    result.push(i as i32);
                    result.push(j as i32);
                    return result;
                }
            }
        }
        result
    }
}

use std::collections::HashMap;
impl Solution4 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (key, value) in nums.iter().enumerate() {
            match map.get(&(target - value)) {
                None => {
                    map.insert(value, key);
                }
                Some(index) => return vec![*index as i32, key as i32],
            }
        }
        vec![]
    }
}

/*
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (i, val) in nums.into_iter().enumerate() {
            if let Some(idx) = map.get(&(target - val)) {
                return vec![*idx as i32, i as i32];
            } else {
                map.insert(val, i);
            }
        }
        vec![0, 0]
    }
}
*/
