fn main() {
    println!("Hello, world!");
}


pub fn mergesort(arr: &mut [i32]) {
    let mid = arr.len() / 2;
    if mid == 0 {                 // 1
        return;
    }

    mergesort(&mut arr[..mid]);   // 2
    mergesort(&mut arr[mid..]);

    // Create an array to store intermediate result.
    let mut ret = arr.to_vec();   // 3

    // Merge the two piles.
    merge(&arr[..mid], &arr[mid..], &mut ret[..]);  // 4

    // Copy back the result back to original array.
    arr.copy_from_slice(&ret);    // 5
}
