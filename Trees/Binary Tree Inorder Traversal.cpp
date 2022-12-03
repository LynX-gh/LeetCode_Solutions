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
    vector<int > res;
    
    void inorderUtil(TreeNode* root){
        if(root == nullptr)
            return;
        
        inorderUtil(root->left);
        res.push_back(root->val);
        inorderUtil(root->right);
    }
    
public:
    vector<int> inorderTraversal(TreeNode* root) {
        inorderUtil(root);
        return res;
    }
};