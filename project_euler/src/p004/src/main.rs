fn main() {
    println!("121: {}",ispalindromic(121));
    println!("1111: {}",ispalindromic(1111));
    println!("900099: {},", ispalindromic(900099));
    println!("906609: {},", ispalindromic(906609));
        
    for i in (100..1000) {
        for j in (100..1000) {
            if ispalindromic(i*j) {
                println!("{} * {} = {}", i, j, i*j);
                return 
            }
        }
    }
}

fn ispalindromic(num: u32) -> bool {
    if num < 10 {
        return true;
    }
        
    let minbit = num - num/10 * 10;
    let mut maxbit = num;
    let mut i = 0;
    while (maxbit > 10) {
        maxbit /=  10;
        i += 1;
    }
    
    if minbit != maxbit {
        return false;
    }
 
    ispalindromic((num - maxbit*(10i32.pow(i) as u32))/10)
}


// there are some bugs !!!!
