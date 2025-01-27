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
    bool hasPathSum(TreeNode* root, int targetSum) {
        if (root == nullptr) {
            return false;
        }
        return helper(root, 0, targetSum);
    }

    bool helper(TreeNode* next, int prevSum, int targetSum) {
        int currSum = prevSum + next->val;

        if(next->right == nullptr && next->left == nullptr && currSum == targetSum) {
            return true;
        } else if (next->right == nullptr && next->left == nullptr && currSum != targetSum) {
            return false;
        } else if (next->left != nullptr && helper(next->left, currSum, targetSum)){
            return true;
        } else if (next->right != nullptr && helper(next->right, currSum, targetSum)){
            return true;
        } else {
            return false;
        }

    }
};