fn main() {
    println!("Hello, world!");
    let a = vec![2, 7, 11, 15];
    println!("{:?}", Solution::two_sum(a,9));
}

struct Solution{}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let len = numbers.len();
        let mut temp = HashMap::new();
        let mut result = vec![0;2];
        
        for i in 0..len {
            if temp.get(&(target-numbers[i])) != None {
                result[0] = *temp.get(&(target-numbers[i])).unwrap() + 1;
                result[1] = i as i32 + 1;
            } else {
                temp.insert(numbers[i], i as i32);
            }
        }
        result
    }
}


/*
执行结果：
通过
显示详情
执行用时 :
0 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
