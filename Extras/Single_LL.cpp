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
	void insert(const int);
	void insend(const int);
	void insord(const int);
	void del();
	void del_after(const int);
	void del_before(const int);
	void display();
};

void MySLL::insert(const int x) {
	ListNode* n = new ListNode(x, head);

	head = n;
}

void MySLL::insend(const int x) {
	ListNode* n = new ListNode(x, nullptr);
	ListNode* temp = head;

	while (temp->next) {
		temp = temp->next;
	}
	temp->next = n;
}

void MySLL::insord(const int x) {
	if (head == nullptr || head->val > x) {
		insert(x);
	}
	else {
		ListNode* temp = head;

		while (temp->next) {
			if (temp->next->val > x) {
				ListNode* n = new ListNode(x, temp->next);
				temp->next = n;
				return;
			}
			temp = temp->next;
		}
		ListNode* n = new ListNode(x, nullptr);
		temp->next = n;
	}
}

void MySLL::del() {
	if (head) {
		ListNode* temp = head;
		head = head->next;
		delete temp;
	}
	else {
		cout << "UnderFlow!" << endl;
	}
}

void MySLL::del_after(const int pos) {
	if (head == nullptr || head->next == nullptr) {
		cout << "UnderFlow!" << endl;
		return;
	}
	else {
		ListNode* temp = head;

		while (temp->next) {
			if (temp->val == pos) {
				ListNode* temp2 = temp->next;
				temp->next = temp2->next;
				delete temp2;
				return;
			}
			temp = temp->next;
		}
		cout << "Element Not Found" << endl;
	}
}

void MySLL::del_before(const int pos) {
	if (head == nullptr || head->next == nullptr || head->val == pos) {
		cout << "UnderFlow!" << endl;
	}
	else if (head->next->val == pos) {
		del();
	}
	else {
		ListNode* temp = head->next;

		while (temp->next->next != nullptr){
			if (temp->next->next->val == pos) {
				ListNode* temp2 = temp->next;
				temp->next = temp2->next;
				delete temp2;
				return;
			}
			temp = temp->next;
		}
		cout << "Element Not Found" << endl;
	}
}

void MySLL::display() {
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
		cout << "\n1. Insert\t\t2. Insend\t\t3. Insord\n4. Delete First\t\t5. Delete After\t\t6. Delete Before\n7. Display\t\t8. Exit" << endl;
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
			sll.del();
			break;
		case 5:
			cout << "Enter the value to delete after : ";
			cin >> temp;
			sll.del_after(temp);
			break;
		case 6:
			cout << "Enter the value to delete before : ";
			cin >> temp;
			sll.del_before(temp);
			break;
		case 7:
			sll.display();
			break;
		case 8:
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