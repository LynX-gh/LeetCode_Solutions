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
    int sumNumbers(TreeNode* root) {
        return recSum(root, 0);
    }

    int recSum(TreeNode* root, int prevSum) {
        int currSum = prevSum*10 + root->val;
        
        if (root->left == nullptr && root->right == nullptr) {
            return prevSum*10 + root->val;
        } else if (root->right == nullptr) {
            return recSum(root->left, prevSum*10 + root->val);
        } else if (root->left == nullptr) {
            return recSum(root->right, prevSum*10 + root->val);
        } else {
            return recSum(root->left, prevSum*10 + root->val) + recSum(root->right, prevSum*10 + root->val);
        }
    }
};