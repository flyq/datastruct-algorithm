

#![allow(unused_variables)]
fn main() {
    let mut a = [9,6,76,6,3,34];
    insertion_sort(&mut a);
    println!("{:?}", a);                 
}


pub fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {                   // 1
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {  // 2
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}
