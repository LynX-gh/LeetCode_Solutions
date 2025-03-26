class Solution {
public:
    vector<int> getRow(int rowIndex) {
        vector<int> res = {1};
        long long prev = 1;
        for (int i = 1; i <= rowIndex; i++) {
            long long next = prev * (rowIndex - i + 1) / i;
            res.push_back(next);
            prev = next;
        }
        return res;
    }
};