class Solution {
public:
    vector<vector<int>> matrixReshape(vector<vector<int>>& mat, int r, int c) {
        int i = mat.size();
        int j = mat[0].size();
        
        if(i*j != r*c)
            return mat;
        
        vector<vector<int>> res(r, vector<int>(c));
        
        for(int i=0; i<(r*c); i++){          
             res[i/c][i%c] = mat[i/j][i%j];
         }

        return res;
    }
};