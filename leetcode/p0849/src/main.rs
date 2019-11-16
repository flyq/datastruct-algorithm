fn main() {
    println!("Hello, world!");
    let a = vec![1,0,0,1,0,1,0,1];
    println!("{}",Solution::max_dist_to_closest(a));
}

pub struct Solution {}

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let len = seats.len() as i32;
        let mut first1 = -1;
        let mut midleft = 0;
        let mut midright = 0;
        let mut max = 0;
        for i in 0..len {
            if seats[i as usize] == 1 {
                if first1 == -1 {
                    first1 = i;
                }
                midright = i;
                if midright-midleft-1 > max {
                    max = midright-midleft-1;
                }
                midleft = midright;
            }
        }
        if first1 >= (max+1)/2 && first1 >= len-midright-1 {
            return first1;
        } else if (max+1)/2 >= first1 && (max+1)/2 >= len-midright-1 {
            return (max+1)/2;
        }
        len-midright-1
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
