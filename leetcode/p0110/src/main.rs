fn main() {
    println!("Hello, world!");
}

pub struct Solution {}

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
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::balanced_helper(root.as_ref()).is_some()
    }

    pub fn balanced_helper(root: Option<&Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(node) = root {
            let pair = (
                Solution::balanced_helper(node.borrow().left.as_ref()),
                Solution::balanced_helper(node.borrow().right.as_ref()),
            );
            match pair {
                (Some(left), Some(right)) => {
                    if i32::abs(left-right) < 2 {
                        return Some(i32::max(left, right) + 1);
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        } else {
            Some(0)
        }
    }
}
