fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    println!("{:?}", binary_search(&a, &2));
    let a = [3];
    println!("{:?}", binary_search(&a, &6));
}

pub fn binary_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
where T: PartialOrd
{
    let mut size = arr.len();
    if size == 0 {
        return Err(0);
    }
    let mut base = 0_usize;

    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        if arr[mid] <= *target {
            base = mid;
        }
        size -= half;
    }

    if arr[base] == *target {
        Ok(base)
    } else {
        Err(base + (arr[base] < *target) as usize)
    }
}

    
