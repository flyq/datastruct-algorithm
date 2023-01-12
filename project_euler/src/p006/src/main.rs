fn main() {
    let mut sum_of_square: u64 = 0;
    let mut sum: u64 = 0;
    for i in 1..101 {
        sum += i;
        sum_of_square += i*i;
    }

    println!("{} - {} = {}", sum*sum, sum_of_square, sum*sum - sum_of_square);
        
}

/*
console:
cargo run 
   Compiling p006 v0.1.0 (/home/flyq/workspaces/flyq/projecteuler/src/p006)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/p006`
25502500 - 338350 = 25164150



website:
Congratulations, the answer you gave to problem 6 is correct.

You are the 457522nd person to have solved this problem.
*/
