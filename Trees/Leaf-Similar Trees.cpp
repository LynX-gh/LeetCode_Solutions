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
    bool leafSimilar(TreeNode* root1, TreeNode* root2) {
        return leafList(root1) == leafList(root2);
    }

    vector<int> leafList(TreeNode* root) {
        vector<int> leaf;
        stack<TreeNode*> stk;
        stk.push(root);

        while(!stk.empty()) {
            TreeNode* curr = stk.top();
            stk.pop();

            if (curr->left == nullptr && curr->right == nullptr) {
                leaf.push_back(curr->val);
            } else {
                if (curr->left != nullptr) {
                    stk.push(curr->left);
                }
                if (curr->right != nullptr) {
                    stk.push(curr->right);
                }
            }
        }
        return leaf;
    }
};