fn main() {
    println!("Hello, world!");
}

pub struct Solution{}


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
       right: None
     }
   }
 }

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node_ref) => {
                Some(Rc::new(RefCell::new(
                    TreeNode {
                        val: node_ref.borrow().val,
                        left: Solution::invert_tree(node_ref.borrow().right.clone()),
                        right: Solution::invert_tree(node_ref.borrow().left.clone()),
                    }
                )))
            },
            None => None,
        }
    }
}


/*
执行用时 :
0 ms
, 在所有 Rust 提交中击败了
100.00%
的用户
内存消耗 :
2 MB
, 在所有 Rust 提交中击败了
71.43%
的用户
*/
