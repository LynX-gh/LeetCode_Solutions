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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut prev = i64::MIN;
        let mut stack = Vec::new();
        let mut curr = root;

        while !stack.is_empty() || curr.is_some() {
            while let Some(node) = curr {
                stack.push(Rc::clone(&node));
                curr = node.borrow().left.as_ref().map(|n| Rc::clone(n));
            }

            if let Some(node) = stack.pop() {
                let node_val = node.borrow().val as i64;
                if prev >= node_val {
                    return false;
                }
                prev = node_val;
                curr = node.borrow().right.as_ref().map(|n| Rc::clone(n));
            }
        }
        true
    }
}