class Node{
public:
    int val;
    Node* prev;
    Node* next;
    
    Node(int x) { val = x; }
};

class MyLinkedList {
public:
    int size;
    Node* head;
    Node* tail;
    
    MyLinkedList() {
        size = 0;
        head = nullptr;
        tail = nullptr;
    }
    
    int get(int index) {
        if(index > size-1) return -1;
        
        Node* cur;
        if(index <= size/2){
            cur = head;
            for(int i=0; i<index; i++){
                cur = cur->next;
            }
        }else{
            cur = tail;
            for(int i=size-1; i>index; i--){
                cur = cur->prev;
            }
        }
        
        return cur->val;
    }
    
    void addAtHead(int val) {
        Node *node = new Node(val);
        
        if(head==nullptr || tail==nullptr){
            head = node;
            tail = node;
        }
        
        node->next = head;
        head->prev = node;
        head = node;
        
        size++;
    }
    
    void addAtTail(int val) {
        Node *node = new Node(val);
        
        if(head==nullptr || tail==nullptr){
            head = node;
            tail = node;
        }
        
        node->prev = tail;
        tail->next = node;
        tail = node;
        
        size++;
    }
    
    void addAtIndex(int index, int val) {
        // index exceeds bounds
        if(index<0 || index>size) return;
        
        // index = 0
        if(index == 0){
            addAtHead(val);
            return;
        }
        
        // index = size
        if(index == size){
            addAtTail(val);
            return;
        }
        
        Node* cur;
        Node* node = new Node(val);
        
        if(index <= size/2){
            cur = head;
            for(int i=0; i<index-1; i++){
                cur = cur->next;
            }
            
            node->next = cur->next;
            cur->next->prev = node;
            cur->next = node;
            node->prev = cur;
            
            size++;
        }else{
            cur = tail;
            for(int i=size-1; i>index; i--){
                cur = cur->prev;
            }
            
            node->prev = cur->prev;
            cur->prev->next = node;
            cur->prev = node;
            node->next = cur;
            
            size++;
        }
    }
    
    void deleteAtIndex(int index) {
        // index exceeds bounds, do nothing
        cout << index << ", " << size << endl;
        if(index<0 || index>=size) return;
        
        // delete head
        if(index == 0){
            head = head->next;
            size--;
            return;
        }
        
        // delete tail
        if(index == size-1){
            tail = tail->prev;
            size--;
            return;
        }
        
        Node* cur;
        Node* temp;
        
        if(index <= size/2){
            cur = head;
            for(int i=0; i<index-1; i++){
                cur = cur->next;
            }
            
            temp = cur->next->next;
            
            cur->next = temp;
            temp->prev = cur;
            
            size--;
        }else{
            cur = tail;
            for(int i=size-1; i>index+1; i--){
                cur = cur->prev;
            }
            
            temp = cur->prev->prev;
            
            cur->prev = temp;
            temp->next = cur;
            
            size--;
        }
        
    }
};

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * MyLinkedList* obj = new MyLinkedList();
 * int param_1 = obj->get(index);
 * obj->addAtHead(val);
 * obj->addAtTail(val);
 * obj->addAtIndex(index,val);
 * obj->deleteAtIndex(index);
 */