fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut result = vec![intervals[0].clone()];
        let mut i = 1;
        while i < intervals.len() {
            let j = result.len();
            if intervals[i][0] <= result[j - 1][1] && intervals[i][1] >= result[j - 1][1] {
                result[j - 1][1] = intervals[i][1];
            } else if intervals[i][0] > result[j - 1][1] {
                result.push(intervals[i].clone());
            }
            i += 1;
        }
        result
    }
}
