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
    bool isPalindrome(ListNode* head) {
        ListNode *fast = head, *slow = head;
        ListNode *rev = nullptr;
        
        while(fast && fast->next){
            fast = fast->next->next;
            ListNode *tmp = slow;
            slow = slow->next;
            tmp->next = rev;
            rev = tmp;
        }
        if(fast!=nullptr){
            slow = slow->next;
        }
        
        while(rev && slow){
            if(rev->val != slow->val){
                return false;
            }
            rev = rev->next;
            slow = slow->next;
        }
        return true;
    }
};