fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn maximum_groups(mut grades: Vec<i32>) -> i32 {
        let len = grades.len();
        let mut i = 0;
        let mut amount = 0;
        while i <= len {
            amount += 1;
            i += amount;
        }
        amount -= 1;
        amount as i32
    }
}
