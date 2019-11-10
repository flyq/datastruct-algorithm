fn main() {
    println!("Hello, world!");
}



pub fn radix_sort(arr: &mut [i32]) {
    let radix = 10;             // 1
    let mut digit = 1;          // 2
    let max_value = arr         // 3
      .iter()
      .max()
      .unwrap_or(&0)
      .clone();
    while digit <= max_value {  // 4
        counting_sort(arr, 0, 9, |t| (t / digit % radix) as usize); // 5
        digit *= radix;         // 6
    }
}
