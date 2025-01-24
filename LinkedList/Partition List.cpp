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
    ListNode* partition(ListNode* head, int x) {
        ListNode *dummy = new ListNode(0, head);
        ListNode *prev = dummy;

        while ( head != nullptr && head->val < x ) {
            head = head->next;
            prev = prev->next;
        }
        
        if (head == nullptr) {
            return dummy->next;
        }

        ListNode *smaller = head->next;
        ListNode *prev_small = head;
        ListNode *temp = nullptr;

        while ( smaller != nullptr ) {
            if (smaller->val < x) {
                prev->next = smaller;
                prev_small->next = smaller->next;
                smaller->next = head;
                prev = prev->next;
                smaller = prev_small->next;
            } else {
                prev_small = smaller;
                smaller = smaller->next;
            }
        }
        return dummy->next;
    }
};