fn main() {
    println!("Hello, world!");
}

pub fn counting_sort<F, T>(arr: &mut [T], min: usize, max: usize, key: F) 
    where F: Fn(&T) -> usize,
          T: Clone,
{
    let mut prefix_sums = {
        // 1. Initialize the count array with default value 0.
        let len = max - min;
        let mut count_arr = Vec::with_capacity(len);
        count_arr.resize(len, 0);

        // 2. Scan elements to collect counts.
        for value in arr.iter() {
            count_arr[key(value)] += 1;
        }

        // 3. Calculate prefix sum.
        count_arr.into_iter().scan(0, |state, x| {
                *state += x;
                Some(*state - x)
            }).collect::<Vec<usize>>()
    };

    // 4. Use prefix sum as index position of output element.
    for value in arr.to_vec().iter() {
        let index = key(value);
        arr[prefix_sums[index]] = value.clone();
        prefix_sums[index] += 1;
    }
}
