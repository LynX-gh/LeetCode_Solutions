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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut count = 0_u32;
        let mut fastptr = &head;
        let mut slowptr = &head;
        while fastptr.is_some() {
            count += 1;
            if count % 2 == 0 {
                slowptr = &slowptr.as_ref().unwrap().next;
            }
            fastptr = &fastptr.as_ref().unwrap().next;
        }
        slowptr.clone()
    }
}