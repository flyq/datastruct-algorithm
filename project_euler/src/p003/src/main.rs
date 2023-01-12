fn main() {
    println!("1's largest prime factor: {}", largestprime(1));
    println!("2's largest prime factor: {}", largestprime(2));
    println!("3's largest prime factor: {}", largestprime(3));
    println!("91's largest prime factor: {}", largestprime(91));    
    println!("13195's largest prime factor: {}", largestprime(13195));
    println!("600851475143's largest prime factor: {}", largestprime(600851475143));    
}

fn largestprime(mut num: u64) -> i64 {
    if num == 0 || num == 1 {
        return -1;
    }

    if num == 2 {
        return 2;
    }

    while num % 2 == 0 {
        num /= 2;
    }

    let upper_limit = (num as f64).sqrt() as u64 + 1;    
    for i in (3..upper_limit).filter(|&x| x % 2 != 0) {      
        while num % i == 0 {
            num /= i;
        }

        if num == 1 {
            num = i;
            break;
        }
    }

    num as i64
}

        
/*
console:
cargo run 
   Compiling p003 v0.1.0 (/home/flyq/workspaces/flyq/projecteuler/src/p003)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/p003`
1's largest prime factor: -1
2's largest prime factor: 2
3's largest prime factor: 3
91's largest prime factor: 13
13195's largest prime factor: 29
600851475143's largest prime factor: 6857


website:
Congratulations, the answer you gave to problem 3 is correct.

You are the 507565th person to have solved this problem.

*/
