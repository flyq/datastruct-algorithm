fn main() {
    println!("Hello, world!");
   
    println!("{:?}", Solution::day_of_the_week(31,1,1971));

    println!("{:?}", Solution::day_of_the_week(1,2,1971));    
    
    println!("{:?}", Solution::day_of_the_week(31,8,2019));    

}
pub struct Solution{}

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let mut days = 0;
        for i in 1971..year {
            if i%400 == 0 || (i%100!=0 && i%4== 0) {
                days += 366;
            } else {
                days += 365;
            }
        }
        for i in 1..month {
            match i {
                1 => days += 31,
                2 => days += 28,
                3 => days += 31,
                4 => days += 30,
                5 => days += 31,
                6 => days += 30,
                7 => days += 31,
                8 => days += 31,
                9 => days += 30,
                10 => days += 31,
                11 => days += 30,
                _ => continue
            }
        }
        if (year%400 == 0 || (year%100!=0 && year%4== 0)) && month > 2 {
            days += 1;
        }
        days += day;
        println!("{}", days);
//        days += 2;
        days %= 7;
        
        match days {
            0 => return "Thursday".to_string(),
            1 => return "Friday".to_string(),
            2 => return "Saturday".to_string(),
            3 => return "Sunday".to_string(),
            4 => return "Monday".to_string(),
            5 => return "Tuesday".to_string(),
            6 => return "Wednesday".to_string(),
            _ => return "".to_string()
        }
        "".to_string()
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
1.9 MB
, 在所有 rust 提交中击败了
100.00%
的用户
 */

