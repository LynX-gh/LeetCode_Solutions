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
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        
        ListNode *fast = head, *slow = head;
        
        // Fast pointer will be ahead by n steps
        for(int i = 0; i < n; i++){
            fast = fast->next;
        }
        // If 1st element needs to be deleted
        if(fast == nullptr){
            head = head->next;
            return head;
        }
        // Else let the pointers reach the end of the list
        else{
            while(fast->next != nullptr){
                fast = fast->next;
                slow = slow->next;
            }

            slow->next = slow->next->next;
            return head;
        }
    }
};