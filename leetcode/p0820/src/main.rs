fn main() {
    println!("Hello, world!");
    let words = vec![String::from("time"), String::from("time"), String::from("time"), String::from("time")];
    let a = Solution::minimum_length_encoding(words);
    println!("{}",a);    
}

pub struct Solution{}
pub struct Solution1{}

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut words_index: Vec<Vec<u8>> = Vec::new();
        let mut all = 0;
        let mut same = 0;

        for i in words {
            let mut index = vec![];
            let temp = i.chars();
            for j in temp {
                index.push(j as u8);
            }
            index.reverse();
            all += index.len()+1;
            words_index.push(index);
        }         
        words_index.sort_unstable_by(|a,b| a.len().partial_cmp(&b.len()).unwrap());
        'outer: for i in 0..words_index.len()-1 {
            'middle: for k in 0..(words_index.len()-i-1) {
                'inner: for j in 0..words_index[i].len() {
                    if words_index[i][j] != words_index[i+1+k][j] {
                        continue 'middle;
                    }
                }
                same += words_index[i].len()+1;
                continue 'outer;
            }
            
        }
        (all-same) as i32        
    }
}


impl Solution1 {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut words_index: Vec<Vec<char>> = Vec::new();
        let mut all = 0;
        let mut same = 0;

        for i in words {
            let mut index = vec![];
            let temp = i.chars();
            for j in temp {
                index.push(j);
            }
            index.reverse();
            all += index.len()+1;
            words_index.push(index);
        }         
        words_index.sort_unstable_by(|a,b| a.len().partial_cmp(&b.len()).unwrap());
        'outer: for i in 0..words_index.len()-1 {
            'middle: for k in 0..(words_index.len()-i-1) {
                'inner: for j in 0..words_index[i].len() {
                    if words_index[i][j] != words_index[i+1+k][j] {
                        continue 'middle;
                    }
                }
                same += words_index[i].len()+1;
                continue 'outer;
            }
            
        }
        (all-same) as i32        
    }
}
