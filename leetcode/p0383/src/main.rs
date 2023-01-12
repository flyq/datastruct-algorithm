fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = [0; 26];

        for c in magazine.as_bytes() {
            map[*c as usize - b'a' as usize] += 1;
        }
        for c in ransom_note.as_bytes() {
            let index = *c as usize - b'a' as usize;
            if map[index] > 0 {
                map[index] -= 1;
            } else {
                return false;
            }
        }
        true
    }
}
