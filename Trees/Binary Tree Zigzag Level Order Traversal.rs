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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut bfs = VecDeque::new();

        if let Some(node) = root {
            bfs.push_back(node);
        }

        let mut level = 0;
        while(!bfs.is_empty()) {
            let mut elems = Vec::new();
            level += 1;
            
            for i in 0..bfs.len() {
                if let Some(curr) = bfs.pop_front() {
                    let curr_ref = curr.borrow();
                    elems.push(curr_ref.val);

                    if let Some(left) = &curr_ref.left {
                        bfs.push_back(left.clone());
                    }
                    if let Some(right) = &curr_ref.right {
                        bfs.push_back(right.clone());
                    }
                }
            }
            if level % 2 == 0 {
                elems.reverse();
            }
            res.push(elems);
        }
        res
    }
}