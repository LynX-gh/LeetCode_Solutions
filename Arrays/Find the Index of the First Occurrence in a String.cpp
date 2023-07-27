class Solution {
public:
    int strStr(string haystack, string needle) {
        if(needle.length() <= haystack.length()){
            int currStart = 0;
            while(currStart <= haystack.length()-needle.length()){
                if(haystack[currStart] == needle[0]){
                    if(haystack.compare(currStart, needle.length(), needle) == 0){
                        return currStart;
                    }
                }
                currStart++;
            }
        }
        return -1;
    }
};