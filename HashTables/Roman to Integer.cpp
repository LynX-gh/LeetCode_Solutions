class Solution {
public:
    int romanToInt(string s) 
    {
        unordered_map<char, int> sym = { { 'I' , 1 }, { 'V' , 5 }, { 'X' , 10 }, { 'L' , 50 }, { 'C' , 100 }, { 'D' , 500 }, { 'M' , 1000 } };
        
        int sum = sym[s.back()];
        for (int i = s.length() - 2; i >= 0; --i) 
        {
            if (sym[s[i]] < sym[s[i + 1]])
                   sum -= sym[s[i]];
            else
                sum += sym[s[i]];
        }
        
        return sum;
    }
};