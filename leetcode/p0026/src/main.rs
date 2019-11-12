fn main() {
    println!("Hello, world!");
    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
    println!("{}", Solution::remove_duplicates(&mut nums));
    println!("{}", Solution1::remove_duplicates(&mut nums));
    println!("{}", Solution2::remove_duplicates(&mut nums));    
    
}


pub struct Solution {}
pub struct Solution1 {}
pub struct Solution2 {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut new_len = nums.len();
        for i in 0..nums.len()-1 {
            if nums[i] == nums[i+1] {
                new_len -= 1;
            }
        }

        let mut j = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[i-1] {
                nums[j] = nums[i];
                j += 1;
            }
        }

        for i in 0..new_len {
            print!("{} ", nums[i]);
        }
        
        new_len as i32
    }
}


// 100%
impl Solution1 {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        
        if len <= 1 {
            return len as i32;
        }

        let mut j = 0;
        for i in 1..len {
            if nums[i] != nums[i-1] {
                j += 1;
                nums[j] = nums[i];
            }
        }        

        
        (j+1usize) as i32
    }
}



// submission codes start here

impl Solution2 {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 1 {
            return len as i32;
        }
        let mut slow = 0usize;
        for fast in 1..len {
            if nums[slow] != nums[fast] {
                slow += 1;
                nums[slow] = nums[fast];
            }
        }
        (slow + 1) as i32
    }
}
