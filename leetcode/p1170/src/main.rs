fn main() {
    println!("Hello, world!");

    let a = vec!["aabbbabaa".to_string()];
    let b = vec!["b".to_string(),"aaaba".to_string(),"aaaabba".to_string(),"aa".to_string(),"aabaabab".to_string(),"aabbaaabbb".to_string(),"ababb".to_string(),"bbb".to_string(),"aabbbabb".to_string(),"aab".to_string(),"bbaaababba".to_string(),"baaaaa".to_string()];
    println!("{:?}", Solution::num_smaller_by_frequency(a, b));
}

pub struct Solution{}

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut res = vec![];
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
            let t = q[i];
            let temp = Solution::binary_search(&w,t);
            res.push(temp);
        }
        res
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
        let mut min = 0;
        let mut max = a.len() - 1;
        if a[max] <= t {
            return 0;
        } 
        while min+1 < max {
            let temp = (min+max)/2;
            if a[temp] > t {
                max = temp;
            } else {
                min = temp;
            }
        }
        (a.len() - max) as i32
    }       
}


/*
执行结果：
通过
显示详情
执行用时 :
4 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
    
