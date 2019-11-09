fn main() {
    println!("Hello, world!");
}


/// Recursion helper
fn quicksort_helper(arr: &mut [i32], lo: isize, hi: isize) {
    if lo <= hi {                               // 1
        let pivot = partition(arr, lo, hi);     // 2
        quicksort_helper(arr, lo, pivot - 1);   // 3
        quicksort_helper(arr, pivot + 1, hi);   // 4
    }
}

fn partition(arr: &mut [i32], lo: isize, hi: isize) -> isize {
    // -- Determine the pivot --
    // In Lomuto parition scheme,
    // the latest element is always chosen as the pivot.
    let pivot = arr[hi as usize];               // 1
    let mut i = lo;

    // -- Swap elements --
    for j in lo..hi {                           // 2
        if arr[j as usize] < pivot {
            arr.swap(i as usize, j as usize);
            i += 1;                             // 3
        }
    }
    // Swap pivot to the middle of two piles.
    arr.swap(i as usize, hi as usize);          // 4
    i // Return the final index of the pivot
}


