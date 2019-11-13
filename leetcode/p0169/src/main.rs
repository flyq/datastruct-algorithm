fn main() {
    println!("Hello, world!");
    let a = vec![2,2,1,1,1,2,2];
    println!("{}", Solution::majority_element(a));
}

struct Solution{}
struct Solution2{}

use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut maxcount = 0;
        let mut maxvalue = 0;
        let mut a = HashMap::new();

        for i in &nums {
            let counter = a.entry(i).or_insert(0);
            *counter += 1;
        }

        for i in &nums {
            let temp = *a.get(i).unwrap();
            if temp > maxcount {
                maxcount = temp;
                maxvalue = *i;
            }
        }
        println!("{:?}", a);
        maxvalue
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
4 ms
, 在所有 rust 提交中击败了
43.59%
的用户
内存消耗 :
2.3 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/



impl Solution2 {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut maxcount = 0;
        let mut maxvalue = 0;
        let mut a = HashMap::new();

        for i in &nums {
            let counter = a.entry(i).or_insert(0);
            *counter += 1;
            if *counter > maxcount {
                maxcount = *counter;
                maxvalue = *i;
            }
        }

        maxvalue
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
2.3 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
