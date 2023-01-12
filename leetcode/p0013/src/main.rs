fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn increment_by(accum: (i32, char), curr: char) -> (i32, char) {
            let (acc, prev) = accum;
            match curr {
                'I' => {
                    if prev == 'V' || prev == 'X' {
                        return (acc - 1, 'I');
                    }
                    return (acc + 1, 'I');
                }
                'V' => (acc + 5, 'V'),
                'X' => {
                    if prev == 'L' || prev == 'C' {
                        return (acc - 10, 'X');
                    }
                    return (acc + 10, 'X');
                }
                'L' => (acc + 50, 'L'),
                'C' => {
                    if prev == 'D' || prev == 'M' {
                        return (acc - 100, 'C');
                    }
                    return (acc + 100, 'C');
                }
                'D' => (acc + 500, 'D'),
                'M' => (acc + 1000, 'M'),
                curr => (acc, curr),
            }
        }
        let r: Vec<char> = s.chars().rev().collect();
        r.iter().fold((0, 'N'), |acc, c| increment_by(acc, *c)).0
    }
}
