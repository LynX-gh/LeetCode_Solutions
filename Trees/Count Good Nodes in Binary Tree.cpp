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
    int goodNodes(TreeNode* root) {
        int res = 0;
        dfs(root, INT_MIN, res);
        return res;
    }

    void dfs(TreeNode* curr, int upper_max, int& res) {
        if (upper_max <= curr->val) {
            res++;
        }

        if (curr->left != nullptr) {
            dfs(curr->left, max(upper_max, curr->val), res);
        }
        if (curr->right != nullptr) {
            dfs(curr->right, max(upper_max, curr->val), res);
        }
    }
};