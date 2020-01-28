fn main() {
    println!("Hello, world!");
}

pub struct Solution {}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
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

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_of_left_leaves_travel(root)
    }

    fn sum_of_left_leaves_travel(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let node = Rc::clone(root.as_ref().unwrap());
        let mut val = 0;

        if node.borrow().left.is_some() {
            let left_node = Rc::clone(node.borrow().left.as_ref().unwrap());
            if left_node.borrow().left.is_none() && left_node.borrow().right.is_none() {
                val += left_node.borrow().val;
            } else {
                val += Self::sum_of_left_leaves_travel(Some(left_node));
            }
        }
        
        if node.borrow().right.is_some() {
            let right_node = Rc::clone(node.borrow().right.as_ref().unwrap());
            
            val += Self::sum_of_left_leaves_travel(Some(right_node));
        }
        
        val
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
2.2 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
