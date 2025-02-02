/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> neighbors;
    Node() {
        val = 0;
        neighbors = vector<Node*>();
    }
    Node(int _val) {
        val = _val;
        neighbors = vector<Node*>();
    }
    Node(int _val, vector<Node*> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};
*/

class Solution {
public:
    Node* cloneGraph(Node* node) {
        if (!node) return nullptr;

        unordered_map<int, Node*> node_map;
        stack<Node*> stk;
        stk.push(node);
        node_map[node->val] = new Node(node->val);

        while (!stk.empty()) {
            Node* curr = stk.top();
            stk.pop();

            for (Node* neighbor : curr->neighbors) {
                if (node_map.find(neighbor->val) == node_map.end()) {
                    node_map[neighbor->val] = new Node(neighbor->val);
                    stk.push(neighbor);
                }
                node_map[curr->val]->neighbors.push_back(node_map[neighbor->val]);
            }
        }

        return node_map[node->val];
    }
};