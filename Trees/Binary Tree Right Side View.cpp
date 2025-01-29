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
    vector<int> rightSideView(TreeNode* root) {
        vector<int> res;

        if(root == nullptr) {
            return res;
        }

        queue<TreeNode*> bfs;
        bfs.push(root);
        
        while(!bfs.empty()) {
            res.push_back(bfs.front()->val);
            for(int i = size(bfs); i > 0; i--){
                TreeNode* curr = bfs.front();
                bfs.pop();

                if(curr->right != nullptr) {
                    bfs.push(curr->right);
                }
                if(curr->left != nullptr) {
                    bfs.push(curr->left);
                }
            }
        }
        return res;
    }
};