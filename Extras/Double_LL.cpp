#include<iostream>

using namespace std;

template<typename T>
class ListNode {
public:
	T val;
	ListNode* prev;
	ListNode* next;
	ListNode(T v, ListNode* ptr1, ListNode* ptr2) :val(v), prev(ptr1), next(ptr2) {}
};

template<typename T>
class MyDLL {
	ListNode<T>* head;
	ListNode<T>* end;
public:
	MyDLL() :head(nullptr), end(nullptr) {}
	void insert(const T);
	void insend(const T);
	void del();
	void del_before(const T);
	void display();
};

template<typename T> void MyDLL<T>::insert(const T x) {
	ListNode<T>* n = new ListNode<T>(x, nullptr, head);

	if (end == nullptr) {
		head = end = n;
		return;
	}

	head->prev = n;
	head = n;
}

template<typename T> void MyDLL<T>::insend(const T x) {
	ListNode<T>* n = new ListNode<T>(x, end, nullptr);

	if (head == nullptr) {
		end = head = n;
		return;
	}

	end->next = n;
	end = n;
}

template<typename T> void MyDLL<T>::del() {
	if (end == nullptr) {
		cout << "UnderFlow!" << endl;
		return;
	}

	if (head == end) {
		delete head;
		head = end = nullptr;
		return;
	}

	ListNode<T>* temp = end;
	end = end->prev;
	end->next = nullptr;
	delete temp;
}

template<typename T> void MyDLL<T>::del_before(const T pos) {
	if (head == nullptr || head == end || head->val == pos) {
		cout << "UnderFlow!" << endl;
		return;
	}

	if (head->next->val == pos) {
		del();
		return;
	}

	ListNode<T>* temp = head->next->next;
	while (temp) {
		if (temp->val == pos) {
			ListNode<T>* temp2 = temp->prev;
			temp->prev = temp2->prev;
			temp2->prev->next = temp;
			delete temp2;
			return;
		}
		temp = temp->next;
	}
	cout << "Element Not Found!" << endl;
}

template<typename T> void MyDLL<T>::display() {
	ListNode<T>* temp = head;
	cout << "Linked List";
	while (temp) {
		cout << " -> " << temp->val;
		temp = temp->next;
	}
	cout << endl;
}

template<typename T> void menu() {
	int choice;
	T temp;
	MyDLL<T> sll;
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
	menu<int>();
}