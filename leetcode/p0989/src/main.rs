fn main() {
    println!("Hello, world!");
    let a = vec![1,2,6,3,0,7,1,7,1,9,7,5,6,6,4,4,0,0,6,3];
    let k = 516;
    println!("{:?} ",  Solution2::add_to_array_form(a, k));
}

pub struct Solution {}
pub struct Solution1 {}
pub struct Solution2 {}

// bug!! for a.len() may == 10000
impl Solution {
    pub fn add_to_array_form(a: Vec<i32>, k: i32) -> Vec<i32> {
        let mut numa = 0i64;
        let lena = a.len() as u32;
        let mut res = vec![];
            
        for i in 0..lena {
            numa += a[i as usize] as i64 * 10i64.pow(lena-1-i);
        }
        numa += k as i64;
        while numa > 0 {
            res.push((numa%10) as i32);
            numa /= 10;
        }
        res.reverse();
        res
    }
}


impl Solution1 {
    pub fn add_to_array_form(a: Vec<i32>, k: i32) -> Vec<i32> {
        let mut a = a;
        let mut arrk = vec![];
        let mut k = k;
        let biglen;
        let mut overflow = 0;
        let mut res = vec![];
        while k > 0 {
            arrk.push(k%10);
            k /= 10;
        }
        a.reverse();
        if a.len() >= arrk.len() {
            biglen = a.len();
        } else {
            biglen = arrk.len();
        }
        for i in 0..biglen {
            if a.get(i) != None && arrk.get(i) != None {
                res.push((a[i]+arrk[i]+overflow) % 10);
                overflow = (a[i]+arrk[i]+overflow)/10;
            } else if a.get(i) != None && arrk.get(i) == None {
                res.push((a[i] + overflow) % 10);
                overflow = (a[i] + overflow) / 10;
            } else if a.get(i) == None && arrk.get(i) != None {
                res.push((arrk[i] + overflow) % 10);
                overflow = (arrk[i] + overflow) / 10;
            }
        }
        if overflow != 0 {
            res.push(overflow);
        }
        res.reverse();
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
66.67%
的用户
内存消耗 :
2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
 */



impl Solution2 {
    pub fn add_to_array_form(a: Vec<i32>, k: i32) -> Vec<i32> {
        let mut a = a;
        let biglen;
        let mut overflow = 0;
        let mut res = vec![];
        let klen = Solution2::num_size(k as u32);
        let mut k = k;
        
        a.reverse();
        if a.len() >= klen {
            biglen = a.len();
        } else {
            biglen = klen;
        }
        for i in 0..biglen {
            let fromk = k%10;
            k /= 10;
            if a.get(i) != None {
                res.push((a[i]+fromk+overflow) % 10);
                overflow = (a[i]+fromk+overflow)/10;
            } else {
                res.push((fromk + overflow) % 10);
                overflow = (fromk + overflow)/10;
            }
        }
        if overflow != 0 {
            res.push(overflow);
        }
        res.reverse();
        res
    }
    pub fn num_size(x: u32) -> usize {
        let a = [10, 100, 1000, 10000, 100000];
        let mut i = 0;
        let mut res = 0;
        while x >= a[i] {
            i += 1;
            res = i+1;
        }
        res
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
12 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
