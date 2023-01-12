fn main() {
    println!("Hello, world!");
    let mut v: Vec<u32> = vec![];

    let mut sum: u64 = 0;
    
    for i in 0..2_000_000 {
        if isprime(i, &mut v) {
            sum += i as u64;
        }
    }

    println!("{}", sum);
    println!("{:?}", v);

        
}


fn isprime(num: u32, v: &mut Vec<u32>) -> bool {
    if num == 0 || num == 1 {
        return false;
    }

    if num % 2 == 0 && num != 2 {
        return false;
    }
   
    for i in v.clone() {
        if num % i == 0 {
            return false;
        }
    }

    v.push(num);
    true
}


/*
我以为这个时间复杂度会更小，但是运行得太慢，即使release也是，可能内存消耗太多
*/
