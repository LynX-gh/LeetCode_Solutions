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
    ListNode* sortList(ListNode* head) {
        if (head == nullptr || head->next == nullptr) {
            return head;
        }

        ListNode* left = head;
        ListNode* mid = getMid(head);
        ListNode* right = mid->next;
        mid->next = nullptr;

        left = sortList(left);
        right = sortList(right);
        return merge(left, right);
    }

    ListNode* getMid(ListNode* head) {
        ListNode* slow = head;
        ListNode* fast = head->next;

        while(fast != nullptr && fast->next != nullptr){
            slow = slow->next;
            fast = fast->next->next;
        }

        return slow;
    }

    ListNode* merge(ListNode* left, ListNode* right) {
        ListNode* dummy = new ListNode(-1);
        ListNode* tail = dummy;

        while(left != nullptr && right != nullptr) {
            if (left->val < right->val) {
                tail->next = left;
                left = left->next;
                tail = tail->next;
            } else {
                tail->next = right;
                right = right->next;
                tail = tail->next;
            }
        }

        while (left != nullptr) {
            tail->next = left;
            left = left->next;
            tail = tail->next;
        }

        while (right != nullptr) {
            tail->next = right;
            right = right->next;
            tail = tail->next;
        }

        return dummy->next;
    }
};