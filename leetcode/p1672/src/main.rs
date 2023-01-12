fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|a| a.iter().sum()).max().unwrap()
    }
}
