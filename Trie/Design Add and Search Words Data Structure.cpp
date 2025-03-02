class TrieNode {
public:
    bool isWord;
    TrieNode* child[26];

    TrieNode(bool word = false) {
        isWord = word;
        for (auto &a : child) a = nullptr;
    }
};

class WordDictionary {
public:
    TrieNode *root;
    WordDictionary() {
        root = new TrieNode();
    }
    
    void addWord(string word) {
        TrieNode* curr = root;
        for (const char c : word) {
            if (curr->child[c - 'a'] == nullptr) {
                curr->child[c - 'a'] = new TrieNode();
            }
            curr = curr->child[c-'a'];
        }
        curr->isWord = true;
    }
    
    bool search(string word) {
        return dfs_search(word, root, 0);
    }

    bool dfs_search(string word, TrieNode* curr, int index) {
        if (index == word.length() - 1 && word[index] == '.') {
            for (auto node : curr->child) {
                if (node != nullptr && node->isWord) {
                    return true;
                }
            }
            return false;
        }

        if (index == word.length() - 1) {
            if (curr->child[word[index] - 'a'] != nullptr) {
                return curr->child[word[index] - 'a']->isWord;
            }
            return false;
        }

        if (word[index] == '.') {
            for (auto node : curr->child) {
                if (node != nullptr && dfs_search(word, node, index+1)) {
                    return true;
                }
            }
            return false;
        }

        if (curr->child[word[index] - 'a'] == nullptr) {
            return false;
        }
        return dfs_search(word, curr->child[word[index] - 'a'], index + 1);
    }
};

/**
 * Your WordDictionary object will be instantiated and called as such:
 * WordDictionary* obj = new WordDictionary();
 * obj->addWord(word);
 * bool param_2 = obj->search(word);
 */