/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
 
func removeNthFromEnd(head *ListNode, n int) *ListNode {
    var curr *ListNode = head;
    var prev *ListNode = nil;

    var end *ListNode = head;
    for i := 1; i < n; i++ {
        end = end.Next;
    }

    for end.Next != nil {
        prev = curr;
        end = end.Next;
        curr = curr.Next;
    }

    if end == head {
        return head.Next;
    } else if prev == nil {
        return curr.Next;
    } else {
        prev.Next = curr.Next;
        return head;
    }
}