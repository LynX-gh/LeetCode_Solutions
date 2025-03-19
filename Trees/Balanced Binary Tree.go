/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func isBalanced(root *TreeNode) bool {
    if root == nil {
        return true;
    }

    diff := depth(root.Left) - depth(root.Right);
    return !(diff < -1 || diff > 1) && isBalanced(root.Left) && isBalanced(root.Right);
}

func depth(root *TreeNode) int {
    if root == nil {
        return 0;
    }
    return max(depth(root.Left), depth(root.Right)) + 1;
}