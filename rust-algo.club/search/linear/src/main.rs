fn main() {
    println!("Hello, world!");
    let a = [1,2,3,4,5,6,7,8,9];

    println!("{}, {}", linear_search(&a, &7).unwrap(), linear_search_2(&a, &7).unwrap());
}

fn linear_search<T>(arr: &[T], target: &T) -> Option<usize>
where T: PartialEq
{
    for (index, item) in arr.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}

fn linear_search_2<T>(arr: &[T], obj: &T) -> Option<usize>
where T: PartialEq
{
    arr.iter().position(|x| x == obj)
}
    
    
