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

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut h = vec![];
        if root == None {
            return true;
        }
        let mut v = vec![];
        v.push(root.unwrap().clone());
        h.push(v);
          while !h.is_empty() {
            let mut vr = Vec::new();
            let mut vh = h.remove(0); //移除第一个
                                      //            println!("vh={:?}", vh);
            let mut vh2 = Vec::new();
            vh.iter().for_each(|t| {
                //每次都是处理下一层是否对称,当前层能走到,说明已经是对称的了.
                if let Some(l) = t.borrow().left.clone() {
                    vr.push(l.borrow().val);
                    vh2.push(l);
                } else {
                    vr.push(-1);
                }
                if let Some(r) = t.borrow().right.clone() {
                    vr.push(r.borrow().val);
                    vh2.push(r)
                } else {
                    vr.push(-1);
                }
            });
            if !vh2.is_empty() {
                h.push(vh2);
            }
            if !Solution::is_symmetric_vec(&vr) {
                return false;
            }
        }
        true
    }

    pub fn is_symmetric_vec(v: &Vec<i32>) -> bool {
        if v.len() % 2 != 0 {
            return false;
        }
        for i in 0..v.len()/2 {
            if v[i] != v[v.len()-1-i] {
                return false;
            }
        }
        true
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
1.9 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
