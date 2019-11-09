
/// Marcin Ciura's gap sequence.
pub const MARCIN_GAPS: [usize; 8] = [701, 301, 132, 57, 23, 10, 4, 1];

fn main() {
    println!("Hello, world!");
}


/// Shellsort
pub fn shellsort(arr: &mut [i32]) {
    let len = arr.len();
    for gap in MARCIN_GAPS.iter() {                     // 1
        let mut i = *gap;                               // 4
        while i < len {                                 // 2
            let mut j = i;
            while j >= *gap && arr[j - gap] > arr[j] {  // 3
                arr.swap(j - *gap, j);
                j -= *gap;
            }
            i += 1;
        }
    }
}
