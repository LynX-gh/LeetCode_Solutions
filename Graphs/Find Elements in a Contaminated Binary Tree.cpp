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
class FindElements {
private:
    unordered_set<int> seen;
public:
    FindElements(TreeNode* root) {
        if (root != nullptr) {
            root->val = 0;
            this->seen.insert(0);
            dfs(root);
        }
    }

    void dfs(TreeNode* head) {
        this->seen.insert(head->val);
        if (head->left != nullptr) {
            head->left->val = head->val * 2 + 1;
            dfs(head->left);
        }
        if (head->right != nullptr) {
            head->right->val = head->val * 2 + 2;
            dfs(head->right);
        }
    }
    
    bool find(int target) {
        return seen.contains(target);
    }
};

/**
 * Your FindElements object will be instantiated and called as such:
 * FindElements* obj = new FindElements(root);
 * bool param_1 = obj->find(target);
 */