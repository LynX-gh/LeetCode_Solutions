class Solution {
public:
    bool canConstruct(string ransomNote, string magazine) {
        vector<int> map1(128, 0);
        vector<int> map2(128, 0);
        bool flag = true;
        
        for(char ch : magazine){
            map1[ch]++;
        }
        
        for(char ch : ransomNote){
            map2[ch]++;
            
            if(map1[ch]<map2[ch])
                flag = false;
        }
        
        return flag;
    }
};