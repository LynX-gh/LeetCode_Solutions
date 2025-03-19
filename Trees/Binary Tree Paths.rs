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
private:
    void dfs(TreeNode* head, string cur, vector<string>& res) {
        cur += "->";
        cur += to_string(head->val);

        if (head->left == nullptr && head->right == nullptr) {
            res.push_back(cur);
        }
        if (head->left != nullptr) {
            dfs(head->left, cur, res);
        }
        if (head->right != nullptr) {
            dfs(head->right, cur, res);
        }
    }
public:
    vector<string> binaryTreePaths(TreeNode* root) {
        vector<string> res;
        if (root != nullptr) {
            if (root->left == nullptr && root->right == nullptr) {
                res.push_back(to_string(root->val));
            }
            if (root->left != nullptr) {
                dfs(root->left, to_string(root->val), res);
            }
            if (root->right != nullptr) {
                dfs(root->right, to_string(root->val), res);
            }
        }
        return res;
    }
};