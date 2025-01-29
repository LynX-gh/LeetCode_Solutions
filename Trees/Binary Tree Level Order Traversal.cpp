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
    vector<vector<int>> levelOrder(TreeNode* root) {
        vector<vector<int>> res;
        queue<TreeNode*> bfs;

        if(root != nullptr) {
            bfs.push(root);
        }

        while(!bfs.empty())
        {
            int sz = bfs.size();
            vector<int> level;

            for(int i=0;i<sz;i++)
            {
                TreeNode* curr = bfs.front();
                bfs.pop();
                level.push_back(curr->val);

                if(curr->left)
                    bfs.push(curr->left);
                if(curr->right)
                    bfs.push(curr->right);
            }
            res.push_back(level);
        }
        return res;
    }
};