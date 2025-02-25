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
    int maxPathSum(TreeNode* root) {
        int res = INT_MIN;
        diameter(root, res);
        return res;
    }

    int diameter(TreeNode* root, int &res) {
        if (root == nullptr) {
            return 0;
        }

        int left = diameter(root->left, res);
        int right = diameter(root->right, res);

        res = max(res, max({left + right, left, right, 0}) + root->val);
        return max({left, right, 0}) + root->val;
    }
};