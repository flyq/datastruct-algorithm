fn main() {
    let a = [1, 2, 3];
    let mut iterator = a.iter();
    println!("{:?}, {:?}, {:?}" , iterator.nth(1), iterator.nth(0), iterator.nth(0));
    //println!("{:?}", iterator.nth(1));
    //println!("{:?}", iterator.nth(0));
}
