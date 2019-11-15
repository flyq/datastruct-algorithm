fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let s:Vec<char> = s.chars().collect();
        let mut min = 0;
        let mut max;
        for i in 1..s.len()+1 {
            if i == s.len() || s[i] != s[i-1] {
                max = (i-1) as i32;
                if max - min >= 2 {
                    res.push(vec![min,max]);
                }
                min = max + 1;
            }
        }
        res
    }
}
