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
    ListNode* reverseKGroup(ListNode* head, int k) {
        int len = 0;
        ListNode* curr = head;
        while(curr != nullptr) {
            len++;
            curr = curr->next;
        }

        for(int i = 1; i + k - 1 <= len; i += k) {
            head = reverseLR(head, i, i + k - 1);
        }
        return head;
    }

    ListNode* reverseLR(ListNode* head, int left, int right) {
        if (head == nullptr || head->next == nullptr || left == right) {
            return head;
        }

        ListNode *curr = head;
        ListNode *prev = nullptr;

        for (int i = 1; i < left; i++) {
            prev = curr;
            curr = curr->next;
        }

        ListNode *connection = prev;
        ListNode* tail = curr;
        ListNode* temp = nullptr;

        for (int i = left; i <= right; i++) {
            temp = curr->next;
            curr->next = prev;
            prev = curr;
            curr = temp;
        }

        tail->next = curr;
        if (connection != nullptr) {
            connection->next = prev;
        } else {
            head = prev;
        }
        return head;
    }
};