#![feature(vec_remove_item)]

fn main() {
    println!("Hello, world!");
    let a = vec![vec![3,10,2,3,4],vec![4,5,6,8,1],vec![8,8,1,6,8],vec![1,3,5,7,1],vec![9,4,9,2,9]];
    println!("{}",Solution1::num_magic_squares_inside(a));
}
pub struct Solution {}
pub struct Solution1 {}
pub struct Solution3 {}

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let rowcount = grid.len();
        let colcount = grid[0].len();
        let a = grid;
        if rowcount < 3 || colcount < 3 {
            return 0;
        }
        let mut amount = 0;
        for i in 0..rowcount {
            for j in 0..colcount {
                if a.get(i+2)!=None && a[i+2].get(j+2)!=None {
                    let mut tempv:Vec<i32> = (1..10).collect();
                    for k in 0..9 {
                        tempv.remove_item(&a[i+k/3][j+k%3]);
                    }
                    if tempv.len() != 0 {
                        break;
                    }
                    let temp1 = a[i][j]+a[i][j+1]+a[i][j+2];
                    let temp2 = a[i+1][j]+a[i+1][j+1]+a[i+1][j+2];
                    let temp3 = a[i+2][j]+a[i+2][j+1]+a[i+2][j+2];
                    let temp4 = a[i][j]+a[i+1][j]+a[i+2][j];
                    let temp5 = a[i][j+1]+a[i+1][j+1]+a[i+2][j+1];
                    let temp6 = a[i][j+2]+a[i+1][j+2]+a[i+2][j+2];
                    let temp7 = a[i][j]+a[i+1][j+1]+a[i+2][j+2];
                    let temp8 = a[i][j+2]+a[i+1][j+1]+a[i+2][j];

                    if a[i][j]!=0 && a[i][j]<10 && temp1==temp2 && temp2==temp3 && temp3==temp4 && temp4==temp5 && temp5==temp6 && temp6==temp7 && temp7==temp8 {
                        amount += 1;
                    }
                }
            }
        }
        amount
    }
}
/*
#![feature(vec_remove_iterm)]
#![feature] may not be used on the stable release channel
 */



use std::collections::HashSet;

impl Solution1 {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let rowcount = grid.len();
        let colcount = grid[0].len();
        let a = grid;
        if rowcount < 3 || colcount < 3 {
            return 0;
        }
        let mut amount = 0;
        'outer: for i in 0..rowcount {
             'inner: for j in 0..colcount {                
                if a.get(i+2)!=None && a[i+2].get(j+2)!=None {
                    let mut tempv = HashSet::new();
                    for k in 0..9 {
                        if a[i+k/3][j+k%3]!=0 && a[i+k/3][j+k%3]<10 {
                            tempv.insert(a[i+k/3][j+k%3]);
                        }
                    }
                    if tempv.len() != 9 {
                        continue 'inner;
                    }
                    let temp1 = a[i][j]+a[i][j+1]+a[i][j+2];
                    let temp2 = a[i+1][j]+a[i+1][j+1]+a[i+1][j+2];
                    let temp3 = a[i+2][j]+a[i+2][j+1]+a[i+2][j+2];
                    let temp4 = a[i][j]+a[i+1][j]+a[i+2][j];
                    let temp5 = a[i][j+1]+a[i+1][j+1]+a[i+2][j+1];
                    let temp6 = a[i][j+2]+a[i+1][j+2]+a[i+2][j+2];
                    let temp7 = a[i][j]+a[i+1][j+1]+a[i+2][j+2];
                    let temp8 = a[i][j+2]+a[i+1][j+1]+a[i+2][j];

                    if temp1==temp2 && temp2==temp3 && temp3==temp4 && temp4==temp5 && temp5==temp6 && temp6==temp7 && temp7==temp8 {
                        amount += 1;
                    }
                }
            }
        }
        amount
    }
}



impl Solution3 {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let rowcount = grid.len();
        let colcount = grid[0].len();
        let a = grid;
        if rowcount < 3 || colcount < 3 {
            return 0;
        }
        let mut amount = 0;
        for i in 0..rowcount {
            for j in 0..colcount {
                if a.get(i+2)!=None && a[i+2].get(j+2)!=None {
                    let mut tempv = HashSet::new();
                    for k in 0..9 {
                        if a[i+k/3][j+k%3]!=0 && a[i+k/3][j+k%3]<10 {
                            tempv.insert(a[i+k/3][j+k%3]);
                        }
                    }
                    if tempv.len() != 9 {
                        continue;
                    }
                    let temp1 = a[i][j]+a[i][j+1]+a[i][j+2];
                    let temp2 = a[i+1][j]+a[i+1][j+1]+a[i+1][j+2];
                    let temp3 = a[i+2][j]+a[i+2][j+1]+a[i+2][j+2];
                    let temp4 = a[i][j]+a[i+1][j]+a[i+2][j];
                    let temp5 = a[i][j+1]+a[i+1][j+1]+a[i+2][j+1];
                    let temp6 = a[i][j+2]+a[i+1][j+2]+a[i+2][j+2];
                    let temp7 = a[i][j]+a[i+1][j+1]+a[i+2][j+2];
                    let temp8 = a[i][j+2]+a[i+1][j+1]+a[i+2][j];

                    if temp1==temp2 && temp2==temp3 && temp3==temp4 && temp4==temp5 && temp5==temp6 && temp6==temp7 && temp7==temp8 {
                        amount += 1;
                    }
                }
            }
        }
        amount
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
2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
