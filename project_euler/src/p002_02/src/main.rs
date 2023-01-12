fn main() {
    let mut value = 0;
    let mut sum = 0;

    let mut pre_1 = 1;
    let mut pre_2 = 2;

    sum += 2; // in 1, 2
    
    // in 3, 5, 8...
    while value < 4_000_000 {
        value = pre_1 + pre_2;

        pre_1 = pre_2;
        pre_2 = value;

        if value / 2 * 2 == value {
            sum += value;
        }
    }

    println!("last value: {}", pre_1);
    println!("sum: {}", sum);
}

/*
console:

cargo run 
   Compiling p002_02 v0.1.0 (/home/flyq/workspaces/flyq/projecteuler/src/p002_02)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `/home/flyq/workspaces/flyq/projecteuler/src/p002_02/target/debug/p002_02`
last value: 3524578
sum: 4613732
*/
