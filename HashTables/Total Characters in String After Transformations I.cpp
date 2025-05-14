class Solution {
public:
    int lengthAfterTransformations(string s, int t) {
        const int modulo = 1e9 + 7;
        vector<int> freq(26, 0);
        for (const char ch: s) {
            freq[int(ch - 'a')] += 1;
        }

        for (int i = 0; i < t; i++) {
            vector<int> temp(26, 0);
            for (int j = 0; j < 25; j++) {
                temp[j+1] = freq[j];
            }
            temp[0] = freq[25] % modulo;
            temp[1] += freq[25] % modulo;
            freq = temp;
        }
        long res = 0;
        for (int i = 0; i < 26; i++){
            res = (res + freq[i]) % modulo;
        }
        return int(res);
    }
};