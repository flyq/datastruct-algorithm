fn main() {
    println!("Hello, world!");
    let g = vec![10,9,8,7];
    let s = vec![5,6,7,8];

    println!("{}", Solution::find_content_children(g,s));
}
pub struct Solution {}

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        if g.len() == 0 || s.len() == 0 {
            return 0;
        }
        
        let mut res = 0;
        let mut g = g;
        let mut s = s;
        g.sort();
        s.sort();
        println!("{:?}, {:?}", g, s);
        
        let mut j = 0;
        'outer: for i in 0..g.len() {
            if j >= s.len() {
                break 'outer;
            }
            
            while g[i] > s[j] {
                j += 1;
                if j >= s.len() {
                    break 'outer;
                }
            }
            j += 1;
            res += 1;
            
        }
        res
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
2.2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
