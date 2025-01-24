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

use std::cmp::max;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_depth_rec(root.as_deref())
    }

    fn max_depth_rec(node: Option<&RefCell<TreeNode>>) -> i32 {
        if let Some(node) = node {
            return i32::max(
                Self::max_depth_rec(node.borrow().left.as_deref()),
                Self::max_depth_rec(node.borrow().right.as_deref()),
            ) + 1;
        }
        0
    }
}