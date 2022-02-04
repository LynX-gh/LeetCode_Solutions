class MyHashSet {
    
    array<vector<int>, 1024> hs;
    
public:
    MyHashSet() {
        
    }
    
    void add(int key) {
        if(!contains(key)){
            hs[key%1024].push_back(key);
        }
    }
    
    void remove(int key) {
        for(auto it = hs[key%1024].begin(); it!=hs[key%1024].end(); it++){
            if(*it == key){
                hs[key%1024].erase(it);
                return;
            }
        }        
    }
    
    bool contains(int key) {
        for(auto k : hs[key%1024]){
            if(k == key)
                return true;
        }
        return false;
    }
};

/**
 * Your MyHashSet object will be instantiated and called as such:
 * MyHashSet* obj = new MyHashSet();
 * obj->add(key);
 * obj->remove(key);
 * bool param_3 = obj->contains(key);
 */