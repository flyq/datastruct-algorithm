fn main() {
    println!("Hello, world!");
    let a = [-1i32, 0, 1, -13];
    let mut iter = a.iter().take_while(|x| x.is_negative());
    assert_eq!(iter.next(), Some(&-1));
    assert_eq!(iter.next(), None);
}
