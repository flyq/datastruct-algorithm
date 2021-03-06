fn main() {
    println!("Hello, world!");
}

pub struct Solution {}

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
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
use std::cmp::max;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (mut h, _) = Self::internal(root);
 
        if h >= 1 {
            h -= 1;
        }
        return h;
    }
  
    fn internal(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if root.is_none() {
            return (0, 0);
        }
        let r = root.unwrap();
        let (l1, l2) = Self::internal(r.borrow().left.clone());
        let (r1, r2) = Self::internal(r.borrow().right.clone());
        let c2 = max(l2, r2) + 1;
        let mut c1 = l2 + r2 + 1;
        c1 = max(c1, l1);
        c1 = max(c1, r1);
        return (c1, c2);
    }
}

/*
执行用时 :
0 ms
, 在所有 Rust 提交中击败了
100.00%
的用户
内存消耗 :
2.6 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
