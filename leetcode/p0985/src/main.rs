fn main() {
    println!("Hello, world!");
    let a = vec![1,2,3,4];
    let b = vec![vec![1,0], vec![-3,1], vec![-4,0], vec![2,3]];
    println!("{:?}", Solution1::sum_even_after_queries(a, b));
}
pub struct Solution{}
pub struct Solution1{}

impl Solution {
    pub fn sum_even_after_queries(a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut a = a;
        let len = a.len();
        let amount = queries.len();
        let mut res = vec![];
        for i in 0..amount {
            let mut sum = 0;
            let index =  queries[i][1] as usize;
            a[index] += queries[i][0];
            for j in 0..len {
                if a[j]%2 == 0 {
                    sum += a[j];
                }
            }
            res.push(sum);
        }
        res
    }
}
/*
执行结果：
通过
显示详情
执行用时 :
116 ms
, 在所有 rust 提交中击败了
60.00%
的用户
内存消耗 :
2.7 MB
, 在所有 rust 提交中击败了
100.00%
的用户
 */

impl Solution1 {
    pub fn sum_even_after_queries(a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut a = a;
        let len = a.len();
        let amount = queries.len();
        let mut res = vec![];
        let mut evensum = 0;
        for i in 0..len {
            if a[i]%2 == 0 {
                evensum += a[i];
            }
        }
        for i in 0..amount {
            let temp;
            let index =  queries[i][1] as usize;
            if a[index]%2 == 0 {
                if (a[index] + queries[i][0])%2 == 0 {
                    temp = evensum + queries[i][0];
                    
                } else {
                    temp = evensum - a[index];
                }
            } else {
                if (a[index] + queries[i][0])%2 == 0 {
                    temp = evensum + queries[i][0] + a[index];
                } else {
                    temp = evensum;
                }
            }
            a[index] += queries[i][0];
            evensum = temp;
            res.push(temp);
        }
        res
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
24 ms
, 在所有 rust 提交中击败了
60.00%
的用户
内存消耗 :
2.7 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
