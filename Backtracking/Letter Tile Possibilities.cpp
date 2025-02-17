class Solution {
public:
    int numTilePossibilities(string tiles) {
        int map[26] = {};
        for (const char ch: tiles) {
            map[ch - 'A']++;
        }

        int res = 0;
        backtrack(map, res);
        return res;
    }

    void backtrack(int map[26], int &res) {
        for (int i = 0; i < 26; i++) {
            if (map[i]) {
                map[i]--;
                res++;
                backtrack(map, res);
                map[i]++;
            }
        }
    }
};