fn main() {
    let mut total: u64 = 1;
    for i in 1..10 {
        total = lcm(total, i+1);
    }
    println!("1 to 10: {}", total);

    //total = 1;
    for i in 1..20 {
        total = lcm(total, i+1);
    }
    println!("1 to 20: {}", total);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a < b {
        let temp = a;
        a = b;
        b = temp;
    }

    if b == 0 {
        return a;
    } else {
        return gcd(b, a%b);
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    return a*b / gcd(a, b);
}


/*
console:

cargo run 
   Compiling p005 v0.1.0 (/home/flyq/workspaces/flyq/projecteuler/src/p005)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/p005`
1 to 10: 2520
1 to 20: 232792560


website:
Congratulations, the answer you gave to problem 5 is correct.

You are the 454560th person to have solved this problem.
*/
