fn main() {
    println!("Hello, world!");
    let a = vec![7,10,1,12,11,14,5,0];
    println!("{}",Solution::distance_between_bus_stops(a, 7, 2));
}
pub struct Solution{}

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let len = distance.len();
        let s;
        let d;
        if start == destination {
            return 0;
        } else if start < destination {
            s = start as usize;
            d = destination as usize;
        } else {
            s = destination as usize;
            d = start as usize;
        }
        let mut all = 0;
        let mut d1 = 0;
        for i in 0..len {
            all += distance[i];
        }
        for i in s..d {
            d1 += distance[i];
        }
        if d1 <= all-d1 {
            return d1;
        }
        all-d1   
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
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
