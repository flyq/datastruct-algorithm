fn main() {
    let a = vec![true; 10];

    for i in 0..a.len() {
        let b = i.checked_sub(1).map_or(false, |j| a[j]);
        println!("b: {}", b);
    }
}
