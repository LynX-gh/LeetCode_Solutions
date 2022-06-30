class Solution {
    int searchRow(vector<vector<int>>& matrix, int target){
        int j = matrix[0].size() - 1;
        
        for(int i = 0; i < matrix.size(); i++){
            if(matrix[i][j] >= target){
                return i;
            }
        }
        
        return -1;
    }
    
public:    
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        int i = searchRow(matrix, target);
        
        if(i==-1)
            return false;
        
        int start = 0;
        int end = matrix[0].size() - 1;
        int mid;
        
        while(end >= start){
            mid = (start+end) / 2;
            if(target > matrix[i][mid])
                start = mid + 1;
            else if (target < matrix[i][mid])
                end = mid - 1;
            else
                return true;
        }
        return false;
    }
};