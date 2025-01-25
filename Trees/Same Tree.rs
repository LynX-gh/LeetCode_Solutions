// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::eq_tree(p.as_deref(), q.as_deref())
    }

    pub fn eq_tree(p: Option<&RefCell<TreeNode>>, q: Option<&RefCell<TreeNode>>) -> bool {
        match (p, q) {
            (Some(node_p), Some(node_q)) => {
                if (node_p.borrow().val == node_q.borrow().val) {
                    let left = Self::eq_tree(node_p.borrow().left.as_deref(), node_q.borrow().left.as_deref());
                    if left == false {
                        return false;
                    }
                    let right = Self::eq_tree(node_p.borrow().right.as_deref(), node_q.borrow().right.as_deref());
                    if right == false {
                        return false;
                    }
                    return true;
                }
                false
            },
            (None, None) => true,
            _ => false
        }
    }
}