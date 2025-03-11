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
    int pairSum(ListNode* head) {
        vector<int> list;
        while (head!=nullptr) {
            list.push_back(head->val);
            head = head->next;
        }
        int res = 0;
        int n = list.size();
        for (int i = 0; i < list.size()/2; i++) {
            if (list[i] + list[n-i-1] > res) {
                res = list[i] + list[n-i-1];
            }
        }
        return res;
    }
};