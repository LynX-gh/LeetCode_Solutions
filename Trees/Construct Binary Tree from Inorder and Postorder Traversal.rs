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
use std::collections::HashMap;

impl Solution {
    pub fn build_tree(mut inorder: Vec<i32>, mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut index_map = HashMap::with_capacity(inorder.len());
        for (idx, val) in inorder.iter().enumerate() {
            index_map.insert(val, idx);
        }
        Self::builder(0, postorder.len()-1, &index_map, &mut postorder)
    }

    fn builder(left: usize, right: usize, index_map: &HashMap<&i32, usize>, mut postorder: &mut Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }

        match postorder.last() {
            Some(val) => {
                let idx = index_map[postorder.last().unwrap()];
                let root = Rc::new(RefCell::new(TreeNode::new(postorder.pop().unwrap())));
                root.borrow_mut().right = Self::builder(idx+1, right, &index_map, &mut postorder);
                root.borrow_mut().left = Self::builder(left, idx-1, &index_map, &mut postorder);
                Some(root)
            },
            None => {
                None
            }
        }
    }
}