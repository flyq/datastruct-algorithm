fn main() {
    println!("Hello, world!");
    let mut nums = vec![0,1,2,2,3,0,4,2];
    println!("{}", Solution3::remove_element(&mut nums, 2));
    
}

struct Solution {}
struct Solution2 {}
struct Solution3 {}

//some bugs!!
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let len = nums.len();
        let mut new_len = len;
        for i in 0..len {
            if nums[i] == val {
                new_len -= 1usize;
            }
        }

        let mut temp = 0;
        for i in 0..new_len {
            if nums[i] == val || temp != 0 {
                for j in temp..(len - new_len + 1) {
                    if nums[i+j] != val {
                        nums[i] = nums[i+j];
                    } else {
                        temp += 1;
                    }
                }
            }
        }
        for i in 0..new_len {
            print!("{} ",nums[i]);
        }
                   
        new_len as i32
    }        
}


impl Solution2 {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index = 0;
        while index < nums.len() {
            if nums[index] == val {
                nums.remove(index);
            }else{
                index += 1;
            }
        }
        nums.len() as i32
    }
}


impl Solution3 {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        for i in 0..nums.len() {
            if i >= nums.len() {
                return nums.len() as i32
            }
            
            if nums[i] == val {
                nums.remove(i);// bug
            }
        }
        nums.len() as i32
    }
}
            
