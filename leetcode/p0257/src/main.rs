fn main() {
    println!("Hello, world!");
}

pub struct Solution {}

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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut res = Vec::new();
        Solution::helper(root, "".to_owned(), &mut res);
        res
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, path: String, res: &mut Vec<String>) {
        if let Some(inner) = root {
            if inner.borrow().left.is_none() && inner.borrow().right.is_none() {
                res.push(format!("{}{}", path, inner.borrow().val));
            } else {
                let path = format!("{}{}->", path, inner.borrow().val);
                Solution::helper(inner.borrow().left.clone(), path.clone(), res);
                Solution::helper(inner.borrow().right.clone(), path, res);
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
2 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
