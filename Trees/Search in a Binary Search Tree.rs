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
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = &root {
            let node_ref = node.borrow();

            if node_ref.val == val {
                return Some(Rc::clone(node));
            } else if node_ref.val > val {
                return Self::search_bst(node_ref.left.clone(), val);
            } else {
                return Self::search_bst(node_ref.right.clone(), val);
            }
        } else {
            return None
        }
    }
}