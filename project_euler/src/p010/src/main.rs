fn main() {
    println!("Hello, world!");
    let mut sum: u64 = 0;
    
    for i in 0..2_000_000 {
        if isprime(i) {
            sum += i as u64;
        }
    }

    println!("{}",sum);
}

fn isprime(num: u32) -> bool {
    if num == 0 || num == 1 {
        return false;
    }

    if num % 2 == 0 && num != 2 {
        return false;
    }
   

    let up_limit = (num as f64).sqrt() as u32 + 1;
    for i in (3..up_limit).filter(|&x| x % 2 != 0) {
        if num % i == 0 {
            return false;
        }
    }

    true
}

/*
console:

cargo run 
   Compiling p010 v0.1.0 (/home/flyq/workspaces/flyq/projecteuler/src/p010)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `/home/flyq/workspaces/flyq/projecteuler/src/p010/target/debug/p010`
Hello, world!
142913828922


website:

Congratulations, the answer you gave to problem 10 is correct.

You are the 303837th person to have solved this problem.

You have earned 1 new award:

Decathlete: Solve ten consecutive problems
*/
        
