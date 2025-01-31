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
    int kthSmallest(TreeNode* root, int k) {
        int elem_idx = 0;
        TreeNode* curr = root;
        stack<TreeNode*> stack;

        while (!stack.empty() || curr != nullptr) {
            while (curr != nullptr) {
                stack.push(curr);
                curr = curr->left;
            }

            curr = stack.top();
            elem_idx++;
            if (elem_idx == k) {
                return curr->val;
            }
            stack.pop();

            curr = curr->right;
        }
        return -1;
    }
};