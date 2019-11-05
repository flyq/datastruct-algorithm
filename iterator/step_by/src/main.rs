fn main() {
    let a = [1, 2, 3, 5, 6, 7];
    let mut iterator = a.iter().step_by(2);
    println!("{:?}, {:?}, {:?}, {:?}" , iterator.next(), iterator.next(), iterator.next(), iterator);
}
