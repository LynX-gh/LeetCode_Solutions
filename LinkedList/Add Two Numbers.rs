// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut sum = 0;
        let mut res = Box::new(ListNode::new(0));
        let mut res_ptr = &mut res;
        let mut l1_ptr = &l1;
        let mut l2_ptr = &l2;

        while l1_ptr.is_some() || l2_ptr.is_some() || carry != 0 {
            sum = match l1_ptr {
                Some(x) => {
                    l1_ptr = &x.next;
                    x.val
                },
                _ => 0,
            } + match l2_ptr {
                Some(x) => {
                    l2_ptr = &x.next;
                    x.val
                },
                _ => 0,
            } + carry;

            carry = sum / 10;

            res_ptr.next = Some(Box::new(ListNode::new(sum % 10)));
            res_ptr = res_ptr.next.as_mut().unwrap();
        }
        res.next
    }
}