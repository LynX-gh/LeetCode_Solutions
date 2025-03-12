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
    int res = 0;
    int longestZigZag(TreeNode* root) {
        dfs(root, true, 0);
        dfs(root, false, 0);
        return res;
    }

    void dfs(TreeNode* curr, bool r, int steps) {
        if (curr == nullptr) {
            return;
        }
        res = max(steps, res);

        if (r) {
            dfs(curr->right, false, steps+1);
            dfs(curr->left, true, 1);
        } else {
            dfs(curr->left, true, steps+1);
            dfs(curr->right, false, 1);
        }
    }
};