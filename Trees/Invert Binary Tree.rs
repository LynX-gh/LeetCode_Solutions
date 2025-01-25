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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                Self::invert_rec(&node);
                Some(node)
            },
            None => None
        }
    }

    fn invert_rec(node: &Rc<RefCell<TreeNode>>) {
        let mut node_ref = node.borrow_mut();
        let left = node_ref.left.take();
        let right = node_ref.right.take();
        
        if let Some(ref left_node) = left {
            Self::invert_rec(left_node);
        }
        if let Some(ref right_node) = right {
            Self::invert_rec(right_node);
        }

        node_ref.left = right;
        node_ref.right = left;
    }
}