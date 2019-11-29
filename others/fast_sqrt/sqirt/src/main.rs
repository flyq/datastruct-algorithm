fn main() {

    println!("sqrt1: {}, ",sqrt1(100f64));
    println!("sqrt2: {}, ",sqrt2(100f64));
    println!("sqrt3: {}, ",sqrt3(100f32));
    //println!("sqrt4: {}, ",sqrt4(100f64));

}

fn sqrt1(x: f64) -> f64 {
    x.sqrt()
}

fn sqrt2(x: f64) -> f64 {
    let mut x = x;
    let xhalf = 0.5*x;
    let mut i = x as i64;
    println!("i in sqrt: {}, ", i);

    i = 0x5f375a86 as i64 - (i>>1);
    
    x = i as f64;
    x = x*(1.5f64 - xhalf*x*x);
    1.0/x
}

fn sqrt3(x: f32) -> f32 {
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);

    let res = y * (1.5 - 0.5 * x * y * y);
    1.0/res
        
}
/*
fn sqrt4(x: f64) -> f64 {
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f64::from_bits(i);

    let res = y * (1.5 - 0.5 * x * y * y);
    1.0/res
        
}
*/
