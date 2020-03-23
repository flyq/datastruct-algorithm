fn main() {
    println!("Hello, world!");
}

pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        if head == None {
            return None;
        }
        let mut len = 0;
        let mut temp = &mut head;
        while !temp.is_none() {
            len += 1;
            temp = &mut temp.as_mut().unwrap().next;
        }
        len /= 2;
        temp = &mut head;
        for i in 0..len {
            temp = &mut temp.as_mut().unwrap().next;
        }
        temp.take()            
    }
}
    
/*
执行结果：
通过
显示详情
执行用时 :
0 ms
, 在所有 Rust 提交中击败了
100.00%
的用户
内存消耗 :
2 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
