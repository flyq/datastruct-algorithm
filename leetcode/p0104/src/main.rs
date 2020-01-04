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
use std::cmp::max;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(root_node_ref) => {
                let root_node = root_node_ref.borrow();
                max(
                    Solution::max_depth(root_node.left.clone()),
                    Solution::max_depth(root_node.right.clone())
                ) + 1
            },
            None => {
                0
            }
        }
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
2.8 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
