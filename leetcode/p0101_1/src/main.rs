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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut result = true;

        root.as_ref().map(|node| {
            result =
                result && Self::is_symmetric_subtrees(&node.borrow().left, &node.borrow().right);
        });

        result
    }

    fn is_symmetric_subtrees(
        p: &Option<Rc<RefCell<TreeNode>>>,
        q: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            true
        } else if (p.is_none() && q.is_some()) || (p.is_some() && q.is_none()) {
            false
        } else {
            let mut result = true;

            p.as_ref().map(|node_p| {
                q.as_ref().map(|node_q| {
                    result = result
                        && node_p.borrow().val == node_q.borrow().val
                        && Self::is_symmetric_subtrees(
                            &node_p.borrow().left,
                            &node_q.borrow().right,
                        )
                        && Self::is_symmetric_subtrees(
                            &node_p.borrow().right,
                            &node_q.borrow().left,
                        );
                });
            });

            result
        }
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
0 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
