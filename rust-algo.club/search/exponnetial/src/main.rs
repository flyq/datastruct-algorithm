use crate::binary::binary_search;

fn main() {
    println!("Hello, world!");
}



pub fn exponential_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
    where T: PartialOrd
{
    // 1. Handle empty scenario.
    let size = arr.len();
    if size == 0 {
        return Err(0);
    }

    // 2. Determine searching boundaries.
    let mut hi = 1_usize; // Upper bound.
    while hi < size && arr[hi] < *target {
        hi <<= 1;
    }
    let lo = hi >> 1; // Lower bound.

    // 3. Do binary search.
    binary_search(&arr[lo..size.min(hi + 1)], target)
        .map(|index| lo + index)
        .map_err(|index| lo + index)
}
