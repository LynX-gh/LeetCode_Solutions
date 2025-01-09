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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = Box::new(ListNode::new(0));
        let mut res_head = &mut res;

        let mut ptr = head.as_ref();

        loop {
            match ptr {
                Some(node) => {
                    if let Some(current) = node.next.as_ref() {
                        res_head.next = Some(Box::new(ListNode::new(current.val)));
                        res_head = res_head.next.as_mut().unwrap();
                        
                        res_head.next = Some(Box::new(ListNode::new(node.val)));
                        res_head = res_head.next.as_mut().unwrap();
                    }
                    else {
                        res_head.next = Some(Box::new(ListNode::new(node.val)));
                        res_head = res_head.next.as_mut().unwrap();
                        return res.next;
                    }
                    ptr = node.next.as_ref().and_then(|n| n.next.as_ref());
                },
                None => {
                    return res.next;
                }
            }
        }
    }
}