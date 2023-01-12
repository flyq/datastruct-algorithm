fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut temp = vec![];
        let len = arr.len();
        let mut max = arr.iter().max().unwrap();
        for i in 0..len - 1 {
            if arr[i] == *max {
                max = arr[i + 1..].iter().max().unwrap();
            }
            temp.push(*max);
        }
        temp.push(-1);
        temp
    }
}
