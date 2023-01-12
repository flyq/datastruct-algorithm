fn main() {
    println!("Hello, world!");
    println!("121: {}",ispalindromic(121));
    println!("1111: {}",ispalindromic(1111));
    println!("900099: {},", ispalindromic(900099));
    println!("906609: {},", ispalindromic(906609));

    let mut max: u32 = 0;
    let mut factor1: u32 = 0;
    let mut factor2: u32 = 0;
    
    for i in (100..1000).rev() {
        for j in (100..(i+1)).rev() {
            if ispalindromic(i*j) && i*j > max {
                max = i*j;
                factor1 = i;
                factor2 = j;
            }
        }
    }

    println!("{}, {}, {}", factor1, factor2, max);
}

fn ispalindromic(mut num: u32) -> bool {
    let mut v: Vec<u32> = Vec::new();

    while num > 0 {
        v.push(num - num/10*10);
        num /= 10;
    }

    if v.len() == 1 || v.len() == 0 {
        return false;
    }

    for i in 0..v.len()/2 {
        if v[i] != v[v.len() - 1 - i] {
            return false
        }
    }

    true
}


/*
console:

cargo run 
   Compiling p004_02 v0.1.0 (/home/flyq/workspaces/flyq/projecteuler/src/p004_02)
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/p004_02`
Hello, world!
121: true
1111: true
900099: false,
906609: true,
993, 913, 906609


website:

Congratulations, the answer you gave to problem 4 is correct.

You are the 449103rd person to have solved this problem.

*/
