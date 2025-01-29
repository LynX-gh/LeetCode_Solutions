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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res = vec![];
        let mut bfs = VecDeque::new();

        if let Some(node) = root {
            bfs.push_back(node);
        }

        while !bfs.is_empty() {
            let mut sum = 0;
            let elems = bfs.len();

            for _ in 0..elems {
                if let Some(curr) = bfs.pop_front() {
                    let curr_ref = curr.borrow();

                    sum += curr_ref.val as i64;

                    if let Some(left) = &curr_ref.left {
                        bfs.push_back(left.clone());
                    }
                    if let Some(right) = &curr_ref.right {
                        bfs.push_back(right.clone());
                    }
                }
            }
            res.push(sum as f64 / elems as f64);
        }
        res
    }
}