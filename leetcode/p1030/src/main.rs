fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];

        for i in 0..r+c+1 {
            let temp = Solution::same_dist(r0,c0,i);
            for j in temp.iter() {
                if j[0]>=0 && j[0]<r && j[1]>=0 && j[1]<c {
                    res.push(j.to_vec());
                }
            }
        }
        res
    }
    pub fn same_dist(r0: i32, c0: i32, d: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        if d == 0 {
            res.push(vec![r0,c0]);
            return res;
        }
        
        let mut j = c0 + d;
        for i in r0..r0+d+1 {
            let point = vec![i,j];
            res.push(point);
            j -= 1;
        }
        
        let mut j = c0 - d;
        for i in r0..r0+d {
            let point = vec![i,j];
            res.push(point);
            j += 1;
        }

        let mut j = c0;
        for i in r0-d..r0 {
            let point = vec![i,j];
            res.push(point);
            j += 1;
        }

        let mut j = c0 - 1;
        for i in r0-d+1..r0 {
            let point = vec![i,j];
            res.push(point);
            j -= 1;
        }
        res
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
36 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.6 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/



// 缩小了遍历的范围
/*
impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let temp = vec![c0+r0, r0+c-c0-1, c0+r-r0-1,r-r0+c-c0-2];
        let mut max = 0;
        for i in temp {
            if i > max {
                max = i;
            }
        }
        for i in 0..max+1 {
            let temp = Solution::same_dist(r0,c0,i);
            for j in temp.iter() {
                if j[0]>=0 && j[0]<r && j[1]>=0 && j[1]<c {
                    res.push(j.to_vec());
                }
            }
        }
        res
    }
    pub fn same_dist(r0: i32, c0: i32, d: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        if d == 0 {
            res.push(vec![r0,c0]);
            return res;
        }
        
        let mut j = c0 + d;
        for i in r0..r0+d+1 {
            let point = vec![i,j];
            res.push(point);
            j -= 1;
        }
        
        let mut j = c0 - d;
        for i in r0..r0+d {
            let point = vec![i,j];
            res.push(point);
            j += 1;
        }

        let mut j = c0;
        for i in r0-d..r0 {
            let point = vec![i,j];
            res.push(point);
            j += 1;
        }

        let mut j = c0 - 1;
        for i in r0-d+1..r0 {
            let point = vec![i,j];
            res.push(point);
            j -= 1;
        }
        res
    }
}

*/
/*
执行结果：
通过
显示详情
执行用时 :
24 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.7 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
