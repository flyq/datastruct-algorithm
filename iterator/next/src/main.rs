fn main() {
    let a = [1, 2, 3];
    let mut iterator = a.iter();
    println!("{:?}, {:?}, {:?}, {:?}", iterator.next(), iterator.next(), iterator.next(), iterator.next());
}
