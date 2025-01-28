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

// O(log^2(n))

class Solution {
private:
    pair<int, int> lrdepth(TreeNode* root) {
        TreeNode* curr = root;
        int ldepth = 0;
        while(curr != nullptr) {
            ldepth++;
            curr = curr->left;
        }

        curr = root;
        int rdepth = 0;
        while(curr != nullptr) {
            rdepth++;
            curr = curr->right;
        }

        return {ldepth, rdepth};
    }
public:
    int countNodes(TreeNode* root) {
        auto [ldepth, rdepth] = lrdepth(root);
        if (ldepth == rdepth) {
            return (1<<rdepth) - 1;
        } else {
            return countNodes(root->left) + 1 + countNodes(root->right);
        }
    }
};

// O(n)
class Solution {
public:
    int countNodes(TreeNode* root) {
		if (root == nullptr) {
			return 0;
		} else {
			return countNodes(root->left) + countNodes(root->right) + 1;
		}
    }
};

class Solution {
public:
    int countNodes(TreeNode* root) {
        if(root == nullptr) {
            return 0;
        } else {
            int nodes = 0;
            stack<TreeNode*> dfs;

            dfs.push(root);
            while(!dfs.empty()) {
                TreeNode* curr = dfs.top();
                dfs.pop();
                nodes++;
                if (curr->left != nullptr)
                    dfs.push(curr->left);
                if (curr->right != nullptr)
                    dfs.push(curr->right);
            }
            return nodes;
        }
    }
};