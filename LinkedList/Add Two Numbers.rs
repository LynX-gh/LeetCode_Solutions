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
        let mut res = Box::new(ListNode::new(0));
        let mut ptr_res = &mut res;
        let mut ptr_l1 = &l1;
        let mut ptr_l2 = &l2;
        let mut sum = 0;
        let mut carry = 0;

        while ptr_l1.is_some() || ptr_l2.is_some() || carry != 0 {
            sum = match ptr_l1 {
                Some(ptr) => {
                    ptr_l1 = &ptr.next;
                    ptr.val
                },
                None => 0
            } + match ptr_l2 {
                Some(ptr) => {
                    ptr_l2 = &ptr.next;
                    ptr.val
                },
                None => 0
            } + carry;

            carry = sum/10;
            ptr_res.next = Some(Box::new(ListNode::new(sum%10)));
            ptr_res = ptr_res.next.as_mut().unwrap();
        }
        res.next
    }
}