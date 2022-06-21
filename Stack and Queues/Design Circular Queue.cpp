class MyCircularQueue {
public:
    vector<int> Queue;
	int f, r, max;
    
    MyCircularQueue(int k):max(k), f(-1), r(-1), Queue(k,0) {}
    
    bool enQueue(int x) {
        if (f == -1) {
            f = r = 0;
            Queue[r] = x;
        }
        else if (r + 1 == max) {
            if (f != 0) {
                r = 0;
                Queue[r] = x;
            }
            else {
                return false;
            }
        }
        else if (r + 1 == f) {
            return false;
        }
        else {
            Queue[++r] = x;
        }
        return true;
    }
    
    bool deQueue() {
        if (f != -1) {
            if (r < f) {
                if (f + 1 < max) {
                    f++;
                }
                else {
                    f = 0;
                }
            }
            else if (r > f) {
                f++;
            }
            else {
                f = r = -1;
            }
            return true;
        }
        else {
            return false;
        }
    }
    
    int Front() {
        if(f == -1)
            return -1;
        else
            return Queue[f];
    }
    
    int Rear() {
        if(r == -1)
            return -1;
        else
            return Queue[r];
    }
    
    bool isEmpty() {
        if(f==-1){
            return true;
        }
        return false;
    }
    
    bool isFull() {
        if(r+1 == max || r+1 == f)
            return true;
        return false;
    }
};

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * MyCircularQueue* obj = new MyCircularQueue(k);
 * bool param_1 = obj->enQueue(value);
 * bool param_2 = obj->deQueue();
 * int param_3 = obj->Front();
 * int param_4 = obj->Rear();
 * bool param_5 = obj->isEmpty();
 * bool param_6 = obj->isFull();
 */