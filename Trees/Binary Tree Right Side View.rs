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
use std::collections::VecDeque;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        // If the root exists, start processing
        if let Some(node) = root {
            queue.push_back(node);
        }

        // Process each level of the tree
        while !queue.is_empty() {
            let level_size = queue.len();

            // Traverse all nodes at the current level
            for i in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    let node_ref = node.borrow();

                    // If this is the last node in the level, add its value to the result
                    if i == level_size - 1 {
                        result.push(node_ref.val);
                    }

                    // Add the left and right children to the queue if they exist
                    if let Some(left) = &node_ref.left {
                        queue.push_back(left.clone());
                    }
                    if let Some(right) = &node_ref.right {
                        queue.push_back(right.clone());
                    }
                }
            }
        }
        result
    }
}