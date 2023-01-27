/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
 /*
class Solution {
public:
    ListNode* middleNode(ListNode* head) {
        float len = 0;
        ListNode* node = head;
        while(node->next != nullptr){
            node = node->next;
            len++;
        }
        int mid = ceil(len/2);
        node = head;
        for(int i = 0; i < mid; i++){
            node = node->next;
        }
        return node;
    }
};
*/
class Solution {
public:
    ListNode* middleNode(ListNode* head) {
        ListNode *fast = head, *slow = head;
        while(fast!=nullptr && fast->next!=nullptr){
            fast = fast->next->next;
            slow = slow->next;
        }
        return slow;
    }
};