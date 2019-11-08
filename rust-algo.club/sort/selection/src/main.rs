fn main() {
    println!("Hello, world!");
}

pub fn selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {                     // 1
        let mut temp = i;
        for j in (i + 1)..len {           // 2
            if arr[temp] > arr[j] {
                temp = j;
            }
        }
        arr.swap(i, temp);                // 3
    }
}
