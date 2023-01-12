fn main() {
    let mut i = 0;
    let mut sum = 0;
    let mut value = 0;
    
    while value <= 4_000_000 {
        value = fibonacci(i);
        if value / 2 * 2 == value {
            sum += value;
        }
        i += 1;
    }
    
    println!("{}", value);
    println!("{}", sum);    
}


fn fibonacci(key: u32) -> i32 {
    let mut value = 0;

    if key == 0 {
        value = 1;
    }
    else if key == 1 {
        value = 2;
    }
    else {
        let mut i = 1;
        let mut pre_1 = 1;
        let mut pre_2 = 2;
        
        while i < key {
            value = pre_1 + pre_2;

            pre_1 = pre_2;
            pre_2 = value;
            i += 1;
        }
    }

    value
}
    

/*
console:
cargo run 
   Compiling p002 v0.1.0 (/home/flyq/workspaces/flyq/projecteuler/src/p002)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `/home/flyq/workspaces/flyq/projecteuler/src/p002/target/debug/p002`
5702887
4613732

website:
Congratulations, the answer you gave to problem 2 is correct.

You are the 710237th person to have solved this problem.
*/
