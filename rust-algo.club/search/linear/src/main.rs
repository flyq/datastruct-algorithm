pub fn linear_search<T>(arr: &[T], target: &T) -> Option<usize>
    where T: PartialEq
{
    for (index, item) in arr.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}


pub fn linear_search2<T>(arr: &[T], obj: &T) -> Option<usize>
    where T: PartialEq
{
    arr.iter().position(|x| x == obj)
}


fn main() {
    let a = [3,5,7,9,500,6,30,66,345];
    b =  linear_search(a, 5) 
}