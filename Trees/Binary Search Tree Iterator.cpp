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
 
class BSTIterator {
private:
    stack<TreeNode*> iter_stack;

    void push_node(TreeNode* root) {
        while(root != nullptr) {
            iter_stack.push(root);
            root = root->left;
        }
    }

public:
    BSTIterator(TreeNode* root) {
        push_node(root);
    }
    
    int next() {
        TreeNode* cur = iter_stack.top();
        iter_stack.pop();
        push_node(cur->right);
        return cur->val;
    }
    
    bool hasNext() {
        return !iter_stack.empty();
    }
};

/**
 * Your BSTIterator object will be instantiated and called as such:
 * BSTIterator* obj = new BSTIterator(root);
 * int param_1 = obj->next();
 * bool param_2 = obj->hasNext();
 */