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
    TreeNode* constructFromPrePost(vector<int>& preorder, vector<int>& postorder) {
        int postIndex = 0;
        int preIndex = 0;
        return buildTree(preorder, postorder, preIndex, postIndex);
    }

private:
    TreeNode* buildTree(vector<int>& preorder, vector<int>& postorder, int& preIndex, int& postIndex) {
        TreeNode* root = new TreeNode(preorder[preIndex++]);

        if (root->val != postorder[postIndex]) {
            root->left = buildTree(preorder, postorder, preIndex, postIndex);
        }
        if (root->val != postorder[postIndex]) {
            root->right = buildTree(preorder, postorder, preIndex, postIndex);
        }

        postIndex++;
        return root;
    }
};