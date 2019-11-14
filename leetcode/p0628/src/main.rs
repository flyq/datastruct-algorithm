fn main() {
    println!("Hello, world!");
}

struct Solution{}

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut temp = nums;
        temp.sort();
        if temp[len-1]<=0 || temp[0]>=0 || len == 3 {
            return temp[len-1]*temp[len-2]*temp[len-3];
        }
        let mut pos = Vec::new();
        let mut neg = Vec::new();
        for i in temp {
            if i < 0 {
                neg.push(i);
            } else {
                pos.push(i);
            }
        }
        pos.sort();
        neg.sort();
        let poslen = pos.len();
        let neglen = neg.len();
        if neglen == 1 {
            return pos[poslen-1]*pos[poslen-2]*pos[poslen-3];
        }
        let max1 = neg[0]*neg[1]*pos[poslen-1];
        let max2 = max1;
        if poslen >= 3 {
            max2 = pos[poslen-1]*pos[poslen-2]*pos[poslen-3];
        }
        if max2 >= max1 {
            return max2;
        }
        max1            
    }
}


/*
执行结果：
通过
显示详情
执行用时 :
12 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
