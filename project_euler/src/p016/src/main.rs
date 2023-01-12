fn main() {
    println!("Hello, world!");

    println!("{:?}", mul_single('5','5'));


    
}

fn mul(f1: &str, f2: &str, result: String) -> String {
    let len1 = f1.len();
    let len2 = f2.len();
    
    let mut over:u8 = 0;
    let mut a = [[u8;len1 + 1]; len2];
    let mut temp:u8 = 0;
    let mut b = [u8; len1 + len2];
    
    for i in 0..len2 {
        for j in 0..len1+1 {
            if j != len1 {
                temp = mul_single(f2[len2-1-i].chars(), f1[len1-j].chars());
            }

            a[i][len1-j-1] = (temp + over)%10;
        
            over = (temp+over)/10;
            temp = 0;
        }
        over = 0;
    }

    for i in 0..len1 + len2 {
        b[len1+len2-1] = a[ 
    
}


fn mul_single(f1: char, f2: char) -> u8 {
    assert_eq!((f1 as u8) > 47 && (f1 as u8) < 58, true);
    assert_eq!((f2 as u8 > 47) && (f2 as u8) < 58, true);

    let multi = (f1 as u8 - 48) * (f2 as u8 - 48);
    multi
}
    
        
            
