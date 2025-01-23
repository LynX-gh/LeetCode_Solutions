/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
 
func hasCycle(head *ListNode) bool {
    if head == nil || head.Next == nil {
        return false
    }

    fast_ptr := head.Next
    slow_ptr := head

    for fast_ptr != nil && fast_ptr.Next != nil {
        if fast_ptr == slow_ptr {
            return true
        }
        fast_ptr = fast_ptr.Next.Next
        slow_ptr = slow_ptr.Next
    }
    return false
}