/*
// Definition for a QuadTree node.
class Node {
public:
    bool val;
    bool isLeaf;
    Node* topLeft;
    Node* topRight;
    Node* bottomLeft;
    Node* bottomRight;
    
    Node() {
        val = false;
        isLeaf = false;
        topLeft = NULL;
        topRight = NULL;
        bottomLeft = NULL;
        bottomRight = NULL;
    }
    
    Node(bool _val, bool _isLeaf) {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = NULL;
        topRight = NULL;
        bottomLeft = NULL;
        bottomRight = NULL;
    }
    
    Node(bool _val, bool _isLeaf, Node* _topLeft, Node* _topRight, Node* _bottomLeft, Node* _bottomRight) {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = _topLeft;
        topRight = _topRight;
        bottomLeft = _bottomLeft;
        bottomRight = _bottomRight;
    }
};
*/

class Solution {
private:
    bool isLeaf(vector<vector<int>>& grid, int y, int x, int n){
        int val = grid[y][x];
        for(int i=0; i<n; i++){
            for(int j=0; j<n; j++){
                if (grid[i+y][j+x] != val)
                    return false;
            }
        }
        return true;
    }

    Node* builder(vector<vector<int>>& grid, int y, int x, int n){
        if (isLeaf(grid,y,x,n)){
            return new Node(grid[y][x],true);
        }
        Node* root = new Node();
        root->topLeft = builder(grid, y, x, n/2);
        root->topRight = builder(grid, y, x+n/2, n/2);
        root->bottomLeft = builder(grid, y+n/2, x, n/2);
        root->bottomRight = builder(grid, y+n/2, x+n/2, n/2);
        return root;
    }

public:
    Node* construct(vector<vector<int>>& grid) {
        return builder(grid, 0, 0, grid.size());
    }
};