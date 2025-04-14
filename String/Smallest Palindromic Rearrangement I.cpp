class Solution {
public:
    string smallestPalindrome(string s) {
        vector<int> map(26, 0);
        for (auto const ch: s) {
            map[ch - 'a']++;
        }
        string first = "", mid = "";
        for (int i = 0; i < 26; i++) {
            if (map[i]%2 == 1 && mid.empty()){
                mid += char('a'+i);
                map[i]--;
            }
            first += string(map[i]/2, char('a'+i));
        }

        string last = first;
        reverse(last.begin(), last.end());
        return first + mid + last;
    }
};
