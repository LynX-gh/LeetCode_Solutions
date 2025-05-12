class Solution {
public:
    void solve(int cur,int k,vector<int> &freq,vector<int> &res){
        if(k==0){
            if(cur%2==0)
                res.push_back(cur);
            return;
        }
        cur=cur*10;
        for(int i=0;i<=9;i++){
            if(cur==0 && i==0)
                continue;
            if(freq[i]>0){
                freq[i]--;
                solve(cur+i,k-1,freq,res);
                freq[i]++;
            }
        }
    }
    vector<int> findEvenNumbers(vector<int>& digits) {
       vector<int> res;
       vector<int> freq(10,0);
       for(auto d:digits)
        freq[d]++; 
        solve(0,3,freq,res);
        return res;
    }
};