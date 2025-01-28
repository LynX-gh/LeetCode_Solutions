/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
    if root == nil {
        return nil
    }

    if root.Val == p.Val || root.Val == q.Val {
        return root
    }

    var left *TreeNode = lowestCommonAncestor(root.Left, p, q)
    var right *TreeNode = lowestCommonAncestor(root.Right, p, q)

    if left != nil && right != nil {
        return root
    } else if right != nil {
        return right
    } else if left != nil {
        return left
    } else {
        return nil
    }
}