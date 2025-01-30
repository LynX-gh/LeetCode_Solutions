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
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn build_tree(mut preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut preorder_dq = VecDeque::from(preorder);
        let mut index_map = HashMap::new();
        for (i, v) in inorder.iter().enumerate() {
            index_map.insert(v, i);
        }

        Self::helper(0, inorder.len()-1, &index_map, &mut preorder_dq)
    }

    fn helper(left: usize, right: usize, index_map: &HashMap<&i32, usize>, mut preorder: &mut VecDeque<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right || left > 3000 || right > 3000 {
            return None;
        }

        match preorder.front() {
            Some(val) => {
                let idx = index_map[preorder.front().unwrap()];
                let root = Rc::new(RefCell::new(TreeNode::new(preorder.pop_front().unwrap())));
                root.borrow_mut().left = Self::helper(left, idx-1, &index_map, &mut preorder);
                root.borrow_mut().right = Self::helper(idx+1, right, &index_map, &mut preorder);
                Some(root)
            },
            None => {
                None
            }
        }
    }
}