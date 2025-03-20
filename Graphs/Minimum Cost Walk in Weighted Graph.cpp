class DSU{
    public:
        DSU(int n){
            parent.resize(n + 1);
            sz.resize(n + 1);
            setAnd.resize(n + 1);
            for(int i = 0; i<=n; i++){
                parent[i] = i;
                sz[i] = 1;
                setAnd[i] = (1 >> 9) - 1;
            }
        }
        int getSetAnd(int a){
            return setAnd[getSet(a)];
        }
        bool isConnected(int a, int b){
            return (getSet(a) == getSet(b));
        }
        void join(int a, int b, int w){
            if(isConnected(a, b)){
                setAnd[getSet(a)] &= w;
                return;
            }
            int pa = getSet(a);
            int pb = getSet(b);
            if(sz[pa] < sz[pb]){
                sz[pb] += sz[pa];
                parent[pa] = pb;
                setAnd[pb] &= setAnd[pa];
                setAnd[pb] &= w;
            }
            else{
                sz[pa] += sz[pb];
                parent[pb] = pa;
                setAnd[pa] &= setAnd[pb];
                setAnd[pa] &= w;
            }
        }
        int getSet(int a){
            if(parent[a] == a)return a;
            return getSet(parent[a]);
        }
        vector<int> parent;
        vector<int> sz;
        vector<int> setAnd;
};
class Solution {
public:
    vector<int> minimumCost(int n, vector<vector<int>>& edges, vector<vector<int>>& query) {
        map<int, int> setAnd;
        DSU dsu(n);
        for(auto &x: edges){
            dsu.join(x[0], x[1], x[2]);
        }
        vector<int> ans;
        for(auto &x: query){
            if(dsu.isConnected(x[0], x[1])){
                ans.push_back(dsu.getSetAnd(x[0]));
            }
            else
                ans.push_back(-1);
        }
        return ans;
    }
};