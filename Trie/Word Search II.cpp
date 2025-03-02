class TrieNode {
public:
    bool isWord;
    TrieNode* child[26];

    TrieNode() {
        isWord = false;
        for (auto &a : child) a = nullptr;
    }
};

class Solution {
public:
    vector<string> findWords(vector<vector<char>>& board, vector<string>& words) {
        vector<string> res;
        TrieNode* root = new TrieNode();
        
        for (const string word : words) {
            addWord(root, word);
        }
        
        unordered_set<string> foundWords;
        for (size_t i = 0; i < board.size(); i++) {
            for (size_t j = 0; j < board[0].size(); j++) {
                string word = "";
                unordered_set<size_t> visited;
                backtrack(board, root, visited, i, j, word, foundWords);
            }
        }
        
        res.assign(foundWords.begin(), foundWords.end());
        return res;
    }

private:
    void addWord(TrieNode* root, const string word) {
        TrieNode* curr = root;
        for (char ch : word) {
            if (!curr->child[ch - 'a']) {
                curr->child[ch - 'a'] = new TrieNode();
            }
            curr = curr->child[ch - 'a'];
        }
        curr->isWord = true;
    }
    
    void backtrack(vector<vector<char>>& board, TrieNode* node, unordered_set<size_t>& visited, int i, int j, string& word, unordered_set<string>& foundWords) {
        if (i < 0 || i >= board.size() || j < 0 || j >= board[0].size()) return;
        
        size_t hash = i * board[0].size() + j;
        if (visited.count(hash)) return;
        
        char ch = board[i][j];
        if (!node->child[ch - 'a']) return;
        
        visited.insert(hash);
        word.push_back(ch);
        node = node->child[ch - 'a'];
        
        if (node->isWord) {
            foundWords.insert(word);
            node->isWord = false;
        }
        
        backtrack(board, node, visited, i + 1, j, word, foundWords);
        backtrack(board, node, visited, i - 1, j, word, foundWords);
        backtrack(board, node, visited, i, j + 1, word, foundWords);
        backtrack(board, node, visited, i, j - 1, word, foundWords);
        
        visited.erase(hash);
        word.pop_back();
    }
};