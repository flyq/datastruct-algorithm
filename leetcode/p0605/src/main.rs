fn main() {
    println!("Hello, world!");
}

struct Solution{}

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut flowerbed = flowerbed;
        let len = flowerbed.len();
        let mut max = 0;
        if len==1 && flowerbed[0]==0 {
            max += 1;
        } else if len >= 2 {
            for i in 0..len {
                if i==0 {
                    if flowerbed[i]==0 && flowerbed[i+1]==0 {
                        max += 1;
                        flowerbed[i] = 1;
                    }
                } else if i>0 && i<len-1 {
                    if flowerbed[i-1]==0 && flowerbed[i]==0 && flowerbed[i+1]==0 {
                        max += 1;
                        flowerbed[i] = 1;
                    }
                } else {
                    if flowerbed[i-1]==0 && flowerbed[i]==0 {
                        max += 1;
                        flowerbed[i] = 1;
                    }
                }
            }
        }
        n<=max                    
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
0 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
