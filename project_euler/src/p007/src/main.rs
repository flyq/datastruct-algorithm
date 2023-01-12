fn main() {
    let mut index: u32 = 1;
    let mut number: u64 = 1;
    loop {
        if isprime(number) {
            index += 1;
        }
        if index == 10001 {
            break;
        }
        
        number += 2;

    }

    println!("10001: {}", number);
    

}

fn isprime(num: u64) -> bool {
    if num == 0 || num == 1 {
        return false;
    }

    if num == 2 {
        return true;
    } else if num % 2 == 0 {
        return false;
    }
    
    let upper_limit = (num as f64).sqrt() as u64 + 1;
    for i in (3..upper_limit).filter(|&x| x % 2 != 0) {
        if num % i == 0 {
            return false;
        }
    }

    true
}


/*
console:
cargo run 
   Compiling p007 v0.1.0 (/home/flyq/workspaces/flyq/projecteuler/src/p007)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `/home/flyq/workspaces/flyq/projecteuler/src/p007/target/debug/p007`
10001: 104743


website:
Congratulations, the answer you gave to problem 7 is correct.

You are the 390884th person to have solved this problem.
*/
