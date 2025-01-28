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

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

struct BSTIterator {
    stack: VecDeque<Rc<RefCell<TreeNode>>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut iter = BSTIterator {
            stack: VecDeque::new(),
        };
        iter.push_node(root);
        iter
    }

    fn push_node(&mut self, node: Option<Rc<RefCell<TreeNode>>>) {
        let mut current = node;
        while let Some(node) = current {
            self.stack.push_back(Rc::clone(&node));
            current = node.borrow().left.as_ref().map(|n| Rc::clone(n));
        }
    }

    fn next(&mut self) -> i32 {
        let node = self.stack.pop_back().expect("No more elements");
        let val = node.borrow().val;
        self.push_node(node.borrow().right.as_ref().map(|n| Rc::clone(n)));
        val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */