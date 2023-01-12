fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut result: Vec<(i32, i32)> = vec![];
        for (idx, val) in mat.into_iter().enumerate() {
            result.push((idx as i32, val.iter().sum()));
        }
        result.sort_by(|a, b| a.1.cmp(&b.1));

        result[0..(k as usize)]
            .iter()
            .map(|a| a.0)
            .collect::<Vec<_>>()
    }
}
