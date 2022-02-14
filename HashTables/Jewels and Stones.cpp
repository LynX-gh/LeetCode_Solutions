class Solution {
public:
    int numJewelsInStones(string jewels, string stones) {
        unordered_set<char> jewel_lookup;
        int result = 0;
        
        for(char c : jewels){
            jewel_lookup.insert(c);
        }
        
        for(char stone : stones){
            if(jewel_lookup.find(stone) != jewel_lookup.end()){ result++; }
        }
        
        return result;
    }
};