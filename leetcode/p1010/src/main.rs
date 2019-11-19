fn main() {
    println!("Hello, world!");
    let a = vec![174,188,377,437,54,498,455,239,183,347,59,199,52,488,147,82];
    println!("{}", Solution1::num_pairs_divisible_by60(a));
}
pub struct Solution{}
pub struct Solution1{}

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut res = 0;
        let len = time.len();
        for i in 0..len {
            for j in i+1..len {
                if (time[i]+time[j])%60 == 0 {
                    res += 1;
                }
            }
        }
        res                    
    }
}
/*
执行结果：
超出时间限制
显示详情
最后执行的输入：
[120,276,496,492,...
查看全部
 */


use std::collections::HashMap;
impl Solution1 {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut time = time;
        let mut map = HashMap::new();
        let mut res = 0;
        for i in 0..time.len() {
            time[i] %= 60;
            let count = map.entry(time[i]).or_insert(0);
            *count += 1;
        }
        for k in map.keys() {
            if *k >= 30 {
                continue;
            }
            let temp = 60 - k;

            if map.get(&temp) != None {
                res += map.get(&temp).unwrap() * map.get(k).unwrap();
            }
        }
        let mut count30 = 0;
        if map.get(&30) != None {
            count30 = *map.get(&30).unwrap();
        }
        let mut count60 = 0;
        if map.get(&0) != None {
            count60 = *map.get(&0).unwrap();
        }
        res + count30*(count30-1)/2 + count60*(count60-1)/2
    }
}

/*
执行用时 :
8 ms
, 在所有 rust 提交中击败了
50.00%
的用户
内存消耗 :
2.3 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
