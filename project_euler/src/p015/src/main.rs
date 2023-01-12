fn main() {
    let mut steps = 1u64;
    
    for i in 0..20 {
        steps = steps * (i+21) / (i+1);
    }

    println!("{}", steps);
}

/*
console:

cargo run 
   Compiling p015 v0.1.0 (/home/flyq/workspaces/flyq/projecteuler/src/p015)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/p015`
137846528820


website:

Congratulations, the answer you gave to problem 15 is correct.

You are the 173206th person to have solved this problem.
*/
