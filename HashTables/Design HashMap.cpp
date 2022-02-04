class MyHashMap {
public:
    std::array<std::list<std::pair<int, int>>, 1024> my_map;
    
    MyHashMap() {
        
    }
    
    void put(int key, int value) {
        auto &list = my_map[key % 1024];
        for (auto & val : list) {
            if (val.first == key) {
                val.second = value;
                return;
            }
        }
        list.emplace_back(key, value);
    }
    
    int get(int key) {
        const auto &list = my_map[key % 1024];
        if (list.empty()) {
            return -1;
        }
        for (const auto & val : list) {
            if (val.first == key) {
                return val.second;
            }
        }
        return -1;
    }
    
    void remove(int key) {
        auto &list = my_map[key % 1024];
        for(auto it = list.begin(); it != list.end(); it++){
            if(it->first == key){ 
                list.erase(it);
                return ;
            }
        }
        return ;
    }
};

/**
 * Your MyHashMap object will be instantiated and called as such:
 * MyHashMap* obj = new MyHashMap();
 * obj->put(key,value);
 * int param_2 = obj->get(key);
 * obj->remove(key);
 */