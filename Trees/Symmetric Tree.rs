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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            Self::check_symmetry(&node.borrow().left, &node.borrow().right)
        } else {
            false
        }
    }

    fn check_symmetry(left: &Option<Rc<RefCell<TreeNode>>>, right: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (Some(left_node), Some(right_node)) => {
                left_node.borrow().val == right_node.borrow().val && 
                Self::check_symmetry(&left_node.borrow().right, &right_node.borrow().left) && 
                Self::check_symmetry(&left_node.borrow().left, &right_node.borrow().right)
            },
            (None, None) => true,
            _ => false
        }
    }
}