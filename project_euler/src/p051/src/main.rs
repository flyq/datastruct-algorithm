fn main() {
    let mut a = [0u32;10];
    
    for i in 50000.. {
        if isprime(i) {
            
        }
    }
     
    println!("{},{},{}", bits(2), bits(8888), bits(1000));
}

fn isprime(num: u32) -> bool {
    if num == 0 || num == 1 {
        return false;
    }

    if num == 2 {
        return true;
    } 

    if num % 2 == 0 {
        return false;
    }

    let uplimit = (num as f64).sqrt() as u32 + 1;
    for i in (3..uplimit).filter(|&x| x % 2 != 0) {
        if num % i == 0 {
            return false;
        }
    }

    true
}

fn bits(mut num: u32) -> u32 {
    let mut bit = 0;
    while num != 0 {
        num /= 10;
        bit += 1;
    }

    return bit;
}
            
