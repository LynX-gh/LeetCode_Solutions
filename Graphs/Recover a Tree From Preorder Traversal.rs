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
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut input = Self::parse_preorder(&traversal);

        Self::dfs_builder(&mut input, -1)
    }

    fn parse_preorder(traversal: &str) -> VecDeque<(i32, i32)> {
        let mut result = VecDeque::new();
        let mut depth = 0;
        let mut num = String::new();
        let mut in_number = false;

        for ch in traversal.chars() {
            if ch == '-' {
                if in_number {
                    result.push_front((num.parse::<i32>().unwrap(), depth));
                    depth = 0;
                    num.clear();
                    in_number = false;
                }
                depth += 1;
            } else {
                if !in_number {
                    in_number = true;
                }
                num.push(ch);
            }
        }

        if !num.is_empty() {
            result.push_front((num.parse::<i32>().unwrap(), depth));
        }

        result
    }

    fn dfs_builder(q: &mut VecDeque<(i32, i32)>, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if q.back().is_some() && q.back().unwrap().1 == depth+1 {
            let (val, d) = q.pop_back().unwrap();
            let mut root = TreeNode::new(val);
            root.left = Self::dfs_builder(q, depth+1);
            root.right = Self::dfs_builder(q, depth+1);
            Some(Rc::new(RefCell::new(root)))
        } else {
            None
        }
    }
}