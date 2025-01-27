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
        if (root == nullptr) {
            return root;
        }

        Node* right_node = nullptr;
        queue<pair<Node*, int>> bfs_queue;
        bfs_queue.push({root, 0});
        int level = 0;

        while (!bfs_queue.empty()) {
            auto curr = bfs_queue.front();
            bfs_queue.pop();

            if (curr.second == level) {
                curr.first->next = right_node;
            } else {
                curr.first->next = nullptr;
                level = curr.second;
            }
            
            if (curr.first->left != nullptr) {
                bfs_queue.push({curr.first->right, level+1});
                bfs_queue.push({curr.first->left, level+1});
            }
            right_node = curr.first;
        }

        return root;
    }
};