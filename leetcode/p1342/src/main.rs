fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut num = num;
        let mut i = 0;
        while num > 0 {
            i += 1;
            if num & 1 == 0 {
                num >>= 1;
            } else {
                num &= !1;
            }
        }
        i
    }
}
