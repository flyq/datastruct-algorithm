fn main() {
    println!("Hello, world!");
}

pub struct Solution {}
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut depth = i32::max_value();
        let node = root.unwrap();
        let mut queue = VecDeque::new();

        queue.push_back((node, 0));

        while !queue.is_empty() {
            let (n, level) = queue.pop_front().unwrap();

            if n.borrow().left.is_some() {
                queue.push_back((Rc::clone(n.borrow().left.as_ref().unwrap()), level + 1));
            }

            if n.borrow().right.is_some() {
                queue.push_back((Rc::clone(n.borrow().right.as_ref().unwrap()), level + 1));
            }
            
            if n.borrow().left.is_none() && n.borrow().right.is_none() {
                if depth >= level {
                    depth = level;
                }
            }
        }
        depth + 1
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
2.4 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
