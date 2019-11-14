fn main() {
    println!("Hello, world!");
    let a = vec![6,3,5,7,2,3,3,8,2,4];
    println!("{}", Solution1::find_pairs(a, 2));
}

struct Solution{}
struct Solution1{}

use std::collections::HashMap;

/* bug!!!
 * 当相距为 2k 的数同时产生中间的数时，会漏掉一个
 */

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        if k < 0 {
            return 0;
        }
        let mut res = 0;
        let mut map = HashMap::new();
        for i in nums {
            let temp = map.get(&i); 
            if temp != None && *temp.unwrap() != 1_0000_0000 {
                res += 1;
                if i > *temp.unwrap() {
                    map.insert(i+k,i);
                } else if i < *temp.unwrap() {
                    map.insert(i-k,i);
                } 
                map.insert(i,1_0000_0000);
            } else if temp == None {
                map.entry(i-k).or_insert(i);
                map.entry(i+k).or_insert(i);
            } 
        }
        res
    }
}


use std::collections::HashSet;
impl Solution1 {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        if k < 0 {
            return 0;
        }
        let mut raw = HashSet::new();
        let mut diff = HashSet::new();

        for i in nums {
            if raw.contains(&(i-k)) {
                diff.insert(i-k);
            }
            if raw.contains(&(i+k)) {
                diff.insert(i);
            }
            raw.insert(i);
        }
        diff.len() as i32
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
8 ms
, 在所有 rust 提交中击败了
62.50%
的用户
内存消耗 :
2.7 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
