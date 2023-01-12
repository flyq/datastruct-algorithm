fn main() {
    println!("under 10: {}", getsum(10));
    println!("under 1000: {}", getsum(1000));
}

fn getsum(max:i32) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    
    while i < max {
        if i / 3 * 3 == i || i / 5 * 5 == i {
            sum += i;
        }

        i += 1;
    }

    sum
}
        

/*
console:
cargo run 
   Compiling p001 v0.1.0 (/home/flyq/workspaces/flyq/projecteuler/src/p001)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `/home/flyq/workspaces/flyq/projecteuler/src/p001/target/debug/p001`
under 10: 23
under 1000: 233168

website:
Congratulations, the answer you gave to problem 1 is correct.

You are the 890619th person to have solved this problem.
*/
