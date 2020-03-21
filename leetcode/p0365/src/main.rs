fn main() {
    println!("Hello, world!");
    println!("{}",Solution::can_measure_water(3,5,4));
}

pub struct Solution{}

impl Solution {
    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        if z == 0 {
            return true;
        }
        if (x == 0 && y != z) || (y == 0 && x != z) || (x+y < z) {
            return false;
        }
        let a = Self::gcd(x,y);
        if z % a != 0 {
            return false;
        }
        true
    }

    fn gcd(a: i32,b: i32) -> i32 {
        if a % b==0 {
            return b;
        }else{
            return Self::gcd(b,a % b);
        }
    }
}


/*
执行结果：
通过
显示详情
执行用时 :
0 ms
, 在所有 Rust 提交中击败了
100.00%
的用户
内存消耗 :
2 MB
, 在所有 Rust 提交中击败了
50.00%
的用户
*/
