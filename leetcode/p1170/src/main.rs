fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut res = vec![0];
        let lenq = queries.len();
        let lenw = words.len();
        let mut q = vec![0;lenq];
        let mut w = vec![0;lenw];
        
        for i in 0..lenq {
            q[i] = Solution::fun(&queries[i]);
        }
        for i in 0..lenw {
            w[i] = Solution::fun(&words[i]);
        }
        w.sort();
        
        for i in 0..lenq {
            let mut j = lenw/2;
            let mut max = 0;
            let mut min = lenw -1;
            while j > min && j < max {
                if words[j] < queries[i] {
                    min = j;
                    j = (min + max)/2;
                } else if words[j] == queries[i] {
                }
            }
        }

        vec![0]
    }
    pub fn fun(a: &String) -> i32 {
        let char_vec: Vec<char> = a.chars().collect();
        let mut min = 'z';
        let mut min_count = 0;
        for i in char_vec {
            if i < min {
                min = i;
                min_count = 1;
            } else if i == min {
                min_count += 1;
            }
        }
        min_count
    }
    pub fn binary_search(a: &Vec<i32>, t: i32) -> i32 {
        
}

