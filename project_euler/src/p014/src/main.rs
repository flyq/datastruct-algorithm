fn main() {
    println!("{}", length(13));

    let mut max = 0;
    let mut num = 0;
    for i in 1..1_000_000 {
        if length(i) > max {
            max = length(i);
            num = i;
        }
    }
    println!("{}, {}", num, max);
        
}

fn length(mut num: u64) -> u32 {
    let mut len = 0;
   
    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
            len += 1;
        } else {
            num = 3*num + 1;
            len += 1;
        }
    }

    len + 1
}

/*
console: 

cargo run 
   Compiling p014 v0.1.0 (/home/flyq/workspaces/flyq/projecteuler/src/p014)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/p014`
Hello, world!
10
837799, 525


website:

Congratulations, the answer you gave to problem 14 is correct.

You are the 210109th person to have solved this problem.
*/
