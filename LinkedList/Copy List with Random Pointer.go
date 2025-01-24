/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Next *Node
 *     Random *Node
 * }
 */

func copyRandomList(head *Node) *Node {
    curr := head;
    node_map := make(map[*Node]*Node)
    
    for curr != nil {
        node := Node{curr.Val, curr.Next, curr.Random}
        node_map[curr] = &node
        curr = curr.Next
    }

    curr = node_map[head]
    for curr != nil {
        curr.Next = node_map[curr.Next]
        curr.Random = node_map[curr.Random]
        curr = curr.Next
    }

    return node_map[head]
}