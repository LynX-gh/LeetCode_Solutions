class Solution {
public:
    bool areAlmostEqual(string s1, string s2) {
        int sw[2] = {0, 0};

        for (int i = 0; i < s1.size(); i++) {
            if (s1[i] != s2[i]) {
                if (sw[0] == 0) {
                    sw[0] = s2[i];
                    sw[1] = s1[i];
                } else if (sw[0] != 1 && sw[0] == s1[i] && sw[1] == s2[i]) {
                    sw[0] = 1;
                } else {
                    return false;
                }
            }
        }
        return ( sw[0] == 0 || sw[0] == 1 );
    }
};