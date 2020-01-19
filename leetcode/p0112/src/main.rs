fn main() {
    println!("Hello, world!");
}

pub struct Solution{}
pub struct Solution2{}

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

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        Solution::has_path_sum_(&root, sum)
    }

    fn has_path_sum_(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                let dif = sum - node.val;
                if node.left.is_none() && node.right.is_none() && dif == 0 {
                    return true;
                }
                Solution::has_path_sum_(&node.left, dif) || Solution::has_path_sum_(&node.right, dif)
            },
            None => false
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
2.5 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
 */
