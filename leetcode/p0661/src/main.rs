fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rowlen = m.len();
        let collen = m[0].len();
        let mut res = vec![vec![0;collen];rowlen];
 
        for i in 0..rowlen {
            for j in 0..collen {
                let mut sum = m[i][j];
                let mut index = 1;
                if m.get(i-1)!=None && m[i-1].get(j-1)!=None {
                    sum += m[i-1][j-1];
                    index += 1;
                }
                if m.get(i-1)!=None && m[i-1].get(j)!=None {
                    sum += m[i-1][j];
                    index += 1;
                }
                if m.get(i-1)!=None && m[i-1].get(j+1)!=None {
                    sum += m[i-1][j+1];
                    index += 1;
                }
                if m.get(i)!=None && m[i].get(j-1)!=None {
                    sum += m[i][j-1];
                    index += 1;
                }
                if m.get(i)!=None && m[i].get(j+1)!=None {
                    sum += m[i][j+1];
                    index += 1;
                }
                if m.get(i+1)!=None && m[i+1].get(j-1)!=None {
                    sum += m[i+1][j-1];
                    index += 1;
                }
                if m.get(i+1)!=None && m[i+1].get(j)!=None {
                    sum += m[i+1][j];
                    index += 1;
                }
                if m.get(i+1)!=None && m[i+1].get(j+1)!=None {
                    sum += m[i+1][j+1];
                    index += 1;
                }
                res[i][j] = sum / index;
            }
        }
        res            
    }
}
