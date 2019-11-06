fn main() {
    let a = [7,8,0, 9];
    let iter = a.iter();
   
    println!("{:?}, {:?}", iter.size_hint(), iter);

    // The even numbers from zero to ten.
    let iter = (0..10).filter(|x| x % 2 == 0);
   
    // We might iterate from zero to ten times. Knowing that it's five
    // exactly wouldn't be possible without executing filter().
    println!("{:?}", iter.size_hint());

    // Let's add five more numbers with chain()
    let iter = (0..10).filter(|x| x % 2 == 0).chain(15..20);
    
    // now both bounds are increased by five
    assert_eq!((5, Some(15)), iter.size_hint());

    println!("{:?}", iter.size_hint());

    let iter = 0..10;
    
    println!("{:?}", iter.size_hint());

    let iter = (0..).filter(|x| x % 2 == 0);
    println!("{:?}", iter.size_hint());        
}
