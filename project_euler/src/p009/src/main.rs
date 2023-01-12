fn main() {
    let mut a = 0;
    let mut b = 0;

    for i in 1..1000 {
        for j in i+1..1000 {
            if 2000*i + 2000*j - 2*i*j == 1000000 {
                println!("{},{}", i, j);
                a = i;
                b = j;
            }
        }
    }

    println!("{}^2 + {}^2 = {}^2 = {}", a, b, ((a*a + b*b) as f64).sqrt(), (a*a + b*b));
    println!("{} + {} + {} = {}", a, b, ((a*a + b*b) as f64).sqrt(), a+b+((a*a + b*b) as f64).sqrt() as i32);
    println!("a*b*c: {}", a*b*((a*a + b*b) as f64).sqrt() as i32);
}

/*
console:

cargo run 
   Compiling p009 v0.1.0 (/home/flyq/workspaces/flyq/projecteuler/src/p009)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `/home/flyq/workspaces/flyq/projecteuler/src/p009/target/debug/p009`
200,375
200^2 + 375^2 = 425^2 = 180625
200 + 375 + 425 = 1000
a*b*c: 31875000

website:

Congratulations, the answer you gave to problem 9 is correct.

You are the 332094th person to have solved this problem.
*/
