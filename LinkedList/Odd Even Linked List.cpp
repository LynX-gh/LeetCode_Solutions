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
    ListNode* oddEvenList(ListNode* head) {
        if (head == nullptr || head->next == nullptr || head->next->next == nullptr) {
            return head;
        }

        ListNode* odd_s = head;
        ListNode* even_s = head->next;
        ListNode* curr_odd = odd_s;
        ListNode* curr_even = even_s;
        ListNode* curr = head->next->next;
        bool odd = true;

        while (curr != nullptr) {
            if (odd) {
                curr_odd->next = curr;
                curr_odd = curr_odd->next;
            } else {
                curr_even->next = curr;
                curr_even = curr_even->next;
            }
            curr = curr->next;
            odd = !odd;
        }
        curr_odd->next = even_s;
        curr_even->next = nullptr;
        return odd_s;
    }
};

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
    ListNode* oddEvenList(ListNode* head) {
        if(head == nullptr || head->next == nullptr || head->next->next == nullptr){
            return head;
        }
        
        ListNode *odd=head, *evenhead=head->next, *even = evenhead;
        while(even && even->next)
        {
            odd->next = odd->next->next;
            even->next = even->next->next;
            odd = odd->next;
            even = even->next;
        }
        odd->next = evenhead;
        return head;
    }
};