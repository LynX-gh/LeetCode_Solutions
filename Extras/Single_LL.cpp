#include<iostream>

using namespace std;

class ListNode {
public:
	int val;
	ListNode* next;
	ListNode(int v, ListNode* ptr) :val(v), next(ptr) {}
};

class MySLL {
	ListNode* head;
public:
	MySLL() :head(nullptr) {}
	void insert(int);
	void insend(int);
	void insord(int);
	void display();
};

void MySLL::insert(int x)
{
	ListNode *n = new ListNode(x, head);

	head = n;
}

void MySLL::insend(int x)
{
	ListNode* n = new ListNode(x, nullptr);
	ListNode* temp = head;

	while (temp->next) {
		temp = temp->next;
	}
	temp->next = n;
}

void MySLL::insord(int x)
{
	if (head->val > x) {
		ListNode* n = new ListNode(x, head);
		head = n;
		return;
	}

	ListNode* temp = head;
	while (temp->next) {
		if (temp->next->val > x) { break; }
		temp = temp->next;
	}

	if (temp->next != nullptr) {
		ListNode* n = new ListNode(x, temp->next);
		temp->next = n;
	}
	else {
		ListNode* n = new ListNode(x, nullptr);
		temp->next = n;
	}
}

void MySLL::display()
{
	ListNode* temp = head;

	cout << "Linked List";
	while (temp) {
		cout << " -> " << temp->val;
		temp = temp->next;
	}
	cout << endl;
}

void menu() {
	int choice, temp;
	MySLL sll;
	while (true) {
		cout << "\n1. Insert\t2. Insend\t\t3. Insord\t\t4. Display\t5. Exit" << endl;
		cin >> choice;
		switch (choice)
		{
		case 1:
			cout << "Enter value to insert : ";
			cin >> temp;
			sll.insert(temp);
			break;
		case 2:
			cout << "Enter value to insert : ";
			cin >> temp;
			sll.insend(temp);
			break;
		case 3:
			cout << "Enter value to insert : ";
			cin >> temp;
			sll.insord(temp);
			break;
		case 4:
			sll.display();
			break;
		case 5:
			exit(0);
		default:
			cout << "Enter a correct choice" << endl;
			cin.clear();
		}
	}
}

int main() {
	menu();
}