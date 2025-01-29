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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut bfs = VecDeque::new();

        if let Some(node) = root {
            bfs.push_back(node);
        }

        while !bfs.is_empty() {
            let elem_size = bfs.len();
            let mut elems = vec![];

            for i in 0..elem_size {
                if let Some(node) = bfs.pop_front() {
                    let node_ref = node.borrow();
                    elems.push(node_ref.val);

                    if let Some(left) = &node_ref.left {
                        bfs.push_back(left.clone());
                    }
                    if let Some(right) = &node_ref.right {
                        bfs.push_back(right.clone());
                    }
                }
            }
            res.push(elems);
        }
        res
    }
}