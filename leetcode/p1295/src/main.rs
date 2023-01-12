fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        nums.into_iter().for_each(|a| {
            if num(a) & 1 == 0 {
                i += 1;
            }
        });
        i
    }
}

fn num(v: i32) -> i32 {
    if v >= 100000 {
        return 6;
    } else if v >= 10000 {
        return 5;
    } else if v >= 1000 {
        return 4;
    } else if v >= 100 {
        return 3;
    } else if v >= 10 {
        return 2;
    } else {
        return 1;
    }
}
