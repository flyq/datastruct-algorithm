fn main() {
    println!("Hello, world!");
}

pub struct Solution{}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32)->Self {
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
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::convert_bst_recurse(&root, 0);
        root
    }
    pub fn convert_bst_recurse(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        match root {
            None => 0,
            Some(node_ref) => {
                let mut node_borrow = node_ref.borrow_mut();
                let mut res = node_borrow.val;
                let right_sum = Solution::convert_bst_recurse(&node_borrow.right, sum);
                node_borrow.val += sum + right_sum;
                res += right_sum;
                let left_sum = Solution::convert_bst_recurse(&node_borrow.left, node_borrow.val);
                res += left_sum;
                res
            }
        }
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
4 ms
, 在所有 Rust 提交中击败了
66.67%
的用户
内存消耗 :
3 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
