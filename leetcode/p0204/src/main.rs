fn main() {
    println!("Hello, world!");
}
pub struct Solution{}
pub struct Solution1{}
pub struct Solution2{}
pub struct Solution3{}

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut res = 0;
        for i in 2..n {
            if Solution::isprime(i) {
                res += 1;
            }
        }
        res
    }

    pub fn isprime(num: i32) -> bool {
        if num < 2 {
            return false;
        }

        if num == 2 {
            return true;
        } else if num % 2 == 0 {
            return false;
        }
        
        let upper_limit = (num as f64).sqrt() as i32 + 1;
        for i in (3..upper_limit).filter(|&x| x % 2 != 0) {
            if num % i == 0 {
                return false;
            }
        }

        true
    }

}
/*
执行结果：
通过
显示详情
执行用时 :
352 ms
, 在所有 rust 提交中击败了
10.00%
的用户
内存消耗 :
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
 */


use std::collections::HashMap;
impl Solution1 {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }
        let mut temp = HashMap::new();
        temp.insert(0,false);
        temp.insert(1,false);

        let upper_limit = (n as f64).sqrt() as i32 + 1;
        for i in 2..upper_limit {
            if temp.get(&i) == None {
                for j in (2*i..n).step_by(i as usize) {
                    temp.insert(j, false);
                }
            }
        }

        n-temp.len() as i32
    }
}
        
/*
执行结果：
通过
显示详情
执行用时 :
644 ms
, 在所有 rust 提交中击败了
10.00%
的用户
内存消耗 :
67.3 MB
, 在所有 rust 提交中击败了
50.00%
的用户
*/





use std::collections::HashSet;
impl Solution2 {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }
        let mut temp = HashSet::new();
        temp.insert(0);
        temp.insert(1);

        let upper_limit = (n as f64).sqrt() as i32 + 1;
        for i in 2..upper_limit {
            if temp.get(&i) == None {
                for j in (2*i..n).step_by(i as usize) {
                    temp.insert(j);
                }
            }
        }

        n-temp.len() as i32
    }
}
       
/*
执行结果：
通过
显示详情
执行用时 :
580 ms
, 在所有 rust 提交中击败了
10.00%
的用户
内存消耗 :
51 MB
, 在所有 rust 提交中击败了
50.00%
的用户
 */


impl Solution3 {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }
        let mut temp = vec![0i32;n as usize];
        temp[0] = 1;
        temp[1] = 1;

        let upper_limit = (n as f64).sqrt() as i32 + 1;
        for i in 2..upper_limit {
            if temp[i as usize] == 0 {
                for j in (2*i..n).step_by(i as usize) {
                    temp[j as usize] = 1;
                }
            }
        }
      
        let res:i32= temp.iter().sum();
        n-res            
    }
}
       
/*
执行结果：
通过
显示详情
执行用时 :
12 ms
, 在所有 rust 提交中击败了
80.00%
的用户
内存消耗 :
7.8 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
