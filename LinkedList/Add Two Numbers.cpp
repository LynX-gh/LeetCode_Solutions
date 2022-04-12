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
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        ListNode* res = new ListNode();
        ListNode* temp = res;
        int carry = 0, sum;
        
        while(l1 != nullptr && l2 != nullptr){
            sum = l1->val + l2->val + carry;
            carry = sum/10;
            
            ListNode* n = new ListNode(sum%10, nullptr);
            temp->next = n;
            temp = temp->next;
            
            l1 = l1->next;
            l2 = l2->next;
        }
        
        while(l1!=nullptr){
            sum = l1->val + carry;
            carry = sum/10;
            
            ListNode *n = new ListNode(sum%10, nullptr);
            temp->next = n;
            temp = temp->next;
            
            l1 = l1->next;
        }
        
        while(l2!=nullptr){
            sum = l2->val + carry;
            carry = sum/10;
            
            ListNode *n = new ListNode(sum%10, nullptr);
            temp->next = n;
            temp = temp->next;
            
            l2 = l2->next;
        }
        
        if(carry>0){
            ListNode *n = new ListNode(carry, nullptr);
            temp->next = n;
            temp = temp->next;
        }
        
        return res->next;
    }
};