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
            right: None
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        
        let mut result = vec![];
        let mut queue: VecDeque<(Option<Rc<RefCell<TreeNode>>>, usize)> = VecDeque::new();
        queue.push_back((root, 0));

        while !queue.is_empty() {
            let (node, level) = queue.pop_front().unwrap();

            if level == result.len() {
                result.push(vec![]);
            }

            if !node.is_none() {
                let n = node.unwrap();
                result[level].push(n.borrow().val);

                if n.borrow().left.is_some() {
                    queue.push_back((
                        Some(Rc::clone(n.borrow().left.as_ref().unwrap())),
                        level + 1,
                    ));
                }
                
                if n.borrow().right.is_some() {
                    queue.push_back((
                        Some(Rc::clone(n.borrow().right.as_ref().unwrap())),
                        level + 1,
                    ));
                }
            }
        }
        result.into_iter().rev().collect()
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
