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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(node) => Self::helper(&node, target_sum),
            None => false
        }
    }

    fn helper(node: &Rc<RefCell<TreeNode>>, target_sum: i32) -> bool {
        let node_ref = node.borrow();
        let current_sum = target_sum - node_ref.val;

        match (&node_ref.left, &node_ref.right) {
            (None, None) => {
                current_sum == 0
            }
            (Some(left), Some(right)) => {
                Self::helper(left, current_sum) || Self::helper(right, current_sum)
            }
            (Some(left), None) => {
                Self::helper(left, current_sum)
            }
            (None, Some(right)) => {
                Self::helper(right, current_sum)
            }
        }
    }
}