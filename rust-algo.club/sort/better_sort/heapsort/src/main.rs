fn main() {
    println!("Hello, world!");
}




pub fn heapsort(arr: &mut [i32]) {
    // -- Heapify part --
    // This procedure would build a valid max-heap.
    // (or min-heap for sorting descendantly)
    let end = arr.len();
    for start in (0..end / 2).rev() {                   // 1
        sift_down(arr, start, end - 1);
    }

    // -- Sorting part --
    // Iteratively sift down unsorted part (the heap).
    for end in (1..arr.len()).rev() {                   // 2
        arr.swap(end, 0);                               // 3
        sift_down(arr, 0, end - 1);                     // 4
    }
}


fn sift_down(arr: &mut [i32], start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1; // Get the left child   // 1
        if child > end {
            break;
        }
        if child + 1 <= end && arr[child] < arr[child + 1] {  // 2
            // Right child exists and is greater.
            child += 1;
        }

        if arr[root] < arr[child] {                           // 3
            // If child is greater than root, swap'em!
            arr.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}
