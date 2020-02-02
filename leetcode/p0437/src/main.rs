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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        Self::path_sum_iii_new_travel(root.as_ref(), sum)
    }
    fn path_sum_iii_new_travel(root: Option<&Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut result = 0;

        root.map(|node| {
            if node.borrow().val == sum {
                result += 1;
            }

            node.borrow().left.as_ref().map(|node_l| {
                result += Self::path_sum_iii_new_travel(Some(node_l), sum)
                    + Self::path_sum_iii_old_travel(Some(node_l), node.borrow().val, sum);
            });

            node.borrow().right.as_ref().map(|node_r| {
                result += Self::path_sum_iii_new_travel(Some(node_r), sum)
                    + Self::path_sum_iii_old_travel(Some(node_r), node.borrow().val, sum);
            });
        });

        result
    }

    fn path_sum_iii_old_travel(root: Option<&Rc<RefCell<TreeNode>>>, sum: i32, target: i32) -> i32 {
        let mut result = 0;

        root.map(|node| {
            if node.borrow().val + sum == target {
                result += 1;
            }

            node.borrow().left.as_ref().map(|node_l| {
                result +=
                    Self::path_sum_iii_old_travel(Some(node_l), sum + node.borrow().val, target);
            });

            node.borrow().right.as_ref().map(|node_r| {
                result +=
                    Self::path_sum_iii_old_travel(Some(node_r), sum + node.borrow().val, target);
            });
        });

        result
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
36 ms
, 在所有 Rust 提交中击败了
57.14%
的用户
内存消耗 :
2.2 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
