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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::helper(&root.unwrap(), 0)
    }

    fn helper(root: &Rc<RefCell<TreeNode>>, prev_sum: i32) -> i32 {
        let curr_sum = prev_sum * 10 + root.borrow().val;

        match (&root.borrow().left, &root.borrow().right) {
            (Some(left), Some(right)) => {
                Self::helper(left, curr_sum) + Self::helper(right, curr_sum)
            },
            (Some(left), _) => {
                Self::helper(left, curr_sum)
            },
            (_, Some(right)) => {
                Self::helper(right, curr_sum)
            },
            (None, None) => {
                curr_sum
            }
        }
    }
}