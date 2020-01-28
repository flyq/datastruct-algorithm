fn main() {
    println!("Hello, world!");
}

pub struct Solution{}


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<NodeTree>>>,
    pub right: Option<Rc<RefCell<NodeTree>>>,
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

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn path_sum_iii(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        Self::path_sum_iii_new_travel(root.as_ref(), sum)
    }


}

