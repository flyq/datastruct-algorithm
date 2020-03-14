fn main() {
    println!("Hello, world!");
}

pub struct Solution{}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut longest = Vec::with_capacity(nums.len());

        for num in nums {
            match longest.binary_search(&num) {
                Err(pos) => {
                    if pos < longest.len() {
                        longest[pos] = num;
                    }  else  {
                        longest.push(num);
                    }
                },
                _ => {}
            }
        }

        longest.len() as i32
    }
}


/*
执行用时 :
0 ms
, 在所有 Rust 提交中击败了
100.00%
的用户
内存消耗 :
2 MB
, 在所有 Rust 提交中击败了
75.00%
的用户
*/
