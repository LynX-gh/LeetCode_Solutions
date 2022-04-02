#include<iostream>

using namespace std;

class ListNode {
public:
	int val;
	ListNode* prev;
	ListNode* next;
	ListNode(int v, ListNode* ptr1, ListNode* ptr2) :val(v), prev(ptr1), next(ptr2) {}
};

class MyDLL {
	ListNode* head;
	ListNode* end;
public:
	MyDLL() :head(nullptr), end(nullptr) {}
	void insert(const int);
	void insend(const int);
	void del();
	void del_before(const int);
	void display();
};

void MyDLL::insert(const int x) {
	ListNode* n = new ListNode(x, nullptr, head);
	if (end) {
		head->prev = n;
		head = n;
	}
	else {
		head = end = n;
	}
}

void MyDLL::insend(const int x) {
	ListNode* n = new ListNode(x, end, nullptr);
	if (head) {
		end->next = n;
		end = n;
	}
	else {
		end = head = n;
	}
}

void MyDLL::del() {
	if (end && head == end) {
		delete head;
		head = end = nullptr;
	}
	else if (end) {
		ListNode* temp = end;
		end = end->prev;
		end->next = nullptr;
		delete temp;
	}
	else {
		cout << "UnderFlow!" << endl;
	}
}

void MyDLL::del_before(const int pos) {
	if (head == nullptr || head == end || head->val == pos) {
		cout << "UnderFlow!" << endl;
	}
	else if (head->next->val == pos) {
		del();
	}
	else {
		ListNode* temp = head->next->next;

		while (temp) {
			if (temp->val == pos) {
				ListNode* temp2 = temp->prev;
				temp->prev = temp2->prev;
				temp2->prev->next = temp;
				delete temp2;
				return;
			}
			temp = temp->next;
		}
		cout << "Element Not Found!" << endl;
	}
}

void MyDLL::display() {
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
	MyDLL sll;
	while (true) {
		cout << "\n1. Insert\t\t2. Insend\n3. Delete\t\t4. Delete Before\n5. Display\t\t6. Exit" << endl;
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
			sll.del();
			break;
		case 4:
			cout << "Enter the value to delete before : ";
			cin >> temp;
			sll.del_before(temp);
			break;
		case 5:
			sll.display();
			break;
		case 6:
			exit(0);
		default:
			cout << "Enter a correct choice" << endl;
			cin.clear();
			cin.ignore(numeric_limits<streamsize>::max(), '\n');
		}
	}
}

int main() {
	menu();
}