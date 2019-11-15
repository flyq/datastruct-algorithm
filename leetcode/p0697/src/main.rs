fn main() {
    println!("Hello, world!");
    let a = vec![1, 2, 2, 3, 1];
    let b = vec![1,2,2,3,1,4,2];
    println!("{}", Solution::find_shortest_sub_array(a));
    println!("{}", Solution::find_shortest_sub_array(b));
    
}
struct Solution{}

use std::collections::HashMap;
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut temp = HashMap::new();
        let mut max = 0;
        let mut items = Vec::new();
        
        for i in 0..nums.len() {
            let stat = temp.entry(nums[i]).or_insert(0);
            *stat += 1;
            if *stat > max {
                max = *stat;
            }
        }

        for (k,v) in &temp {
            if v == &max {
                items.push(k);
            }
        }

        let mut minindex = 0;
        let mut maxindex = 0;
        let mut minlen;
        let mut res = 500000;
        for i in items {
            for j in 0..nums.len() {
                if nums[j] == *i {
                    minindex = j;
                    break;
                }
            }
            for j in (0..nums.len()).rev() {
                if nums[j] == *i {
                    maxindex = j;
                    break;
                }
            }
            minlen = maxindex - minindex + 1;
            if res > minlen {
                res = minlen;
            }
        }
        res as i32
    }
}


/*
执行结果：
通过
显示详情
执行用时 :
32 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.3 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
