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
class Solution {
public:
    ListNode* removeElements(ListNode* head, int val) {
        if(head==nullptr){ return head; }
        
        if(head->val == val){
            while(head && head->val == val){
                head = head->next;
            }
        }
        
        ListNode* tmp = head;
        while(tmp && tmp->next){
            if(tmp->next->val == val){
                tmp->next = tmp->next->next;
            }
            else{
                tmp = tmp->next;
            }
        }
        return head;
    }
};