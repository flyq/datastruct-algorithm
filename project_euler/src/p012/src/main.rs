fn main() {
    for i in 1.. {
        if num_of_factor(triange(i)) > 500 {
            println!("{}, {}", triange(i), num_of_factor(triange(i)));
            break;
        }
    }
}

fn triange(index: u32) -> u32 {
    (index + 1)*index/2
}

fn num_of_factor(a: u32) -> u32 {
    let mut num = 0;
    
    if a == 0 {
        return 0;
    }
    if a == 1 {
        return 1;
    }

    for i in 2..(a as f64).sqrt() as u32 + 1 {
        if a%i == 0 {
            if a == i*i {
                num += 1;
            } else {
                num += 2;
            }
        }
    }

    num+2
}

/*
console:

cargo run 
   Compiling p012 v0.1.0 (/home/flyq/workspaces/flyq/projecteuler/src/p012)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/p012`
76576500, 576

website:

Congratulations, the answer you gave to problem 12 is correct.

You are the 204524th person to have solved this problem.
*/
