/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */

func rotateRight(head *ListNode, k int) *ListNode {
    if (head == nil || head.Next == nil) {
        return head
    }
    
    var len = 1
    var end *ListNode = head
    for end.Next != nil {
        len++
        end = end.Next
    }
    end.Next = head

    var pos = k % len

    var curr *ListNode = head
    var prev *ListNode = head
    for i := 0; i < len-pos; i++ {
        prev = curr
        curr = curr.Next
    }

    prev.Next = nil
    return curr
}