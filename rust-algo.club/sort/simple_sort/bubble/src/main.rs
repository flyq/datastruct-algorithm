fn main() {
    println!("Hello, world!");
}

pub fn bubble_sort(arr: &mut [i32]) {
    let mut swapped = true;                 // 1
    while swapped {
        swapped = false;
        for i in 1..arr.len() {             // 2
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true              // 3
            }
        }
    }
}
