fn main() {
    println!("Hello, world!");
    println!("sqrt1: {}, ",sqrt2(100f64));
}

fn sqrt1(x: f64) -> f64 {
    x.sqrt()
}

fn sqrt2(x: f64) -> f64 {
    let mut x = x;
    let xhalf = 0.5*x;
    let mut i = x as i64;
    println!("sqrt1: {}, ", i);

    i = 0x5f375a86 as i64 - (i>>1);
    
    x = i as f64;
    x = x*(1.5f64 - xhalf*x*x);
    1.0/x
}
