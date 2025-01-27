/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Left *Node
 *     Right *Node
 *     Next *Node
 * }
 */
 
type NodeLevel struct {
    Node  *Node
    Level int
}

func connect(root *Node) *Node {
    if root == nil {
        return root
    }

    var rightNode *Node
    bfsQueue := list.New()
    bfsQueue.PushBack(&NodeLevel{Node: root, Level: 0})
    level := 0

    for bfsQueue.Len() > 0 {
        front := bfsQueue.Front()
        bfsQueue.Remove(front)
        curr := front.Value.(*NodeLevel)

        if curr.Level == level {
            curr.Node.Next = rightNode
        } else {
            curr.Node.Next = nil
            level = curr.Level
        }

        if curr.Node.Right != nil {
            bfsQueue.PushBack(&NodeLevel{Node: curr.Node.Right, Level: level + 1})
        }
        if curr.Node.Left != nil {
            bfsQueue.PushBack(&NodeLevel{Node: curr.Node.Left, Level: level + 1})
        }        

        rightNode = curr.Node
    }

    return root
}