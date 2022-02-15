class RandomizedSet {
public:
    unordered_set<int> storage;
    RandomizedSet() {

    }
    
    bool insert(int val) {
        if(storage.find(val) == storage.end()){
            storage.insert(val);
            return true;
        }
        else
            return false;
    }
    
    bool remove(int val) {
        if(storage.find(val) != storage.end()){
            storage.erase(val);
            return true;
        }
        else
            return false;
    }
    
    int getRandom() {
        int rnd = rand() % storage.size();
        auto itr = storage.begin();
        advance(itr, rnd);
        return *itr;
    }
};

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * RandomizedSet* obj = new RandomizedSet();
 * bool param_1 = obj->insert(val);
 * bool param_2 = obj->remove(val);
 * int param_3 = obj->getRandom();
 */