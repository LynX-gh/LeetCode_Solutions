/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    int sumOfLeftLeaves(TreeNode* root) {
        if (root == nullptr) {
            return 0;
        }
        return dfs(root, false);
    }

    int dfs(TreeNode* node, bool left) {
        if (node->left == nullptr && node->right == nullptr) {
            if (left) return node->val;
            else return 0;
        }

        int ls = 0;
        int rs = 0;
        if (node->left != nullptr) {
            ls = dfs(node->left, true);
        }
        if (node->right != nullptr) {
            rs = dfs(node->right, false);
        }
        return ls + rs;
    }
};
