class Solution {
public:
    string addStrings(string num1, string num2) {
        string ans;
        int i=num1.size()-1,j=num2.size()-1;
        int extra=0;
        while(i>=0 || j>=0 || extra>0)
        {
            int val1 = (i >= 0) ? num1[i] - '0' : 0;
            int val2 = (j >= 0) ? num2[j] - '0' : 0;
            int sum=val1+val2+extra;
            ans+=(sum%10)+'0';
            extra=sum/10;
            i--;
            j--;
        }
        reverse(ans.begin(),ans.end());
        return ans;
    }
};
