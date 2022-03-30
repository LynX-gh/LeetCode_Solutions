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
    ListNode* reverseList(ListNode* head) {
        if(head == nullptr){return head;}
        ListNode *end = head;
        ListNode *start = head;
        ListNode *temp = head;
        while(end->next != nullptr){
            temp = start;
            start = end->next;
            end->next = start->next;
            start->next = temp;
        }
        return start;
    }
};