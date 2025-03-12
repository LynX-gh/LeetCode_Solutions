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
    int maxLevelSum(TreeNode* root) {
        int res = 0;
        int cur_sum = 0;
        int max_sum = INT_MIN;
        int d = 1;

        queue<TreeNode*> q;
        if (root != nullptr) {
            q.push(root);
        }

        while(!q.empty()) {
            for (int i = q.size(); i > 0; i--) {
                TreeNode* curr = q.front();
                q.pop();
                cur_sum += curr->val;

                if (curr->right != nullptr) {
                    q.push(curr->right);
                }
                if (curr->left != nullptr) {
                    q.push(curr->left);
                }
            }
            if (cur_sum > max_sum) {
                res = d;
                max_sum = cur_sum;
            }
            d++;
            cur_sum = 0;
        }
        return res;
    }
};
