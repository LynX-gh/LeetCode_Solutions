class Solution {
public
    string reverseVowels(string s) {
        if (s.length() == 1) return s;
        int i = 0;
        int j = s.length();
        
        while (i  j) {
            while(ij && s[i] !='A' && s[i] !='E' && s[i] !='I' && s[i] !='O' && s[i] !='U' && s[i] !='a' && s[i] !='e' && s[i] !='i' && s[i] !='o' && s[i] !='u'
            ){
                i++;
            }
            while(ij && s[j] !='A' && s[j] !='E' && s[j] !='I' && s[j] !='O' && s[j]!='U' && s[j] != 'a' && s[j] !='e'&& s[j] !='i' && s[j] !='o' && s[j] !='u'
            ){
                j--;
            }

            if (i  j) {
                swap(s[i], s[j]);
                i++;
                j--;
            }
        }
        return s;
    }
};