/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* left;
    Node* right;
    Node* next;

    Node() : val(0), left(NULL), right(NULL), next(NULL) {}

    Node(int _val) : val(_val), left(NULL), right(NULL), next(NULL) {}

    Node(int _val, Node* _left, Node* _right, Node* _next)
        : val(_val), left(_left), right(_right), next(_next) {}
};
*/

#include<queue>

class Solution {
public:
    Node* connect(Node* root) {
        if(root==nullptr) {
            return root;
        }

        queue<pair<int, Node*>> bfs_q;
        Node* right_node = nullptr;
        int level = 0;
        bfs_q.push({0, root});

        while (!bfs_q.empty()) {
            auto curr = bfs_q.front();
            bfs_q.pop();

            if (curr.first == level) {
                curr.second->next = right_node;
            } else {
                curr.second->next = nullptr;
                level = curr.first;
            }

            if (curr.second->right) {
                bfs_q.push({level+1, curr.second->right});
            }
            if (curr.second->left) {
                bfs_q.push({level+1, curr.second->left});
            }

            right_node = curr.second;
        }
        return root;
    }
};