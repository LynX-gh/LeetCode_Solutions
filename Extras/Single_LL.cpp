#include<iostream>

using namespace std;

template<typename T>
class ListNode {
public:
	T val;
	ListNode* next;
	ListNode(T v, ListNode* ptr) :val(v), next(ptr) {}
};

template<typename T>
class MySLL {
	ListNode<T>* head;
public:
	MySLL() :head(nullptr) {}
	void insert(const T);
	void insend(const T);
	void insord(const T);
	void del();
	void del_after(const T);
	void del_before(const T);
	void display();
};

template<typename T>
void MySLL<T>::insert(const T x) {
	ListNode<T>* n = new ListNode<T>(x, head);

	head = n;
}

template<typename T>
void MySLL<T>::insend(const T x) {
	ListNode<T>* n = new ListNode<T>(x, nullptr);
	ListNode<T>* temp = head;

	while (temp->next) {
		temp = temp->next;
	}
	temp->next = n;
}

template<typename T>
void MySLL<T>::insord(const T x) {
	if (head == nullptr || head->val > x) {
		insert(x);
	}
	else {
		ListNode<T>* temp = head;

		while (temp->next) {
			if (temp->next->val > x) {
				ListNode<T>* n = new ListNode<T>(x, temp->next);
				temp->next = n;
				return;
			}
			temp = temp->next;
		}
		ListNode<T>* n = new ListNode<T>(x, nullptr);
		temp->next = n;
	}
}

template<typename T>
void MySLL<T>::del() {
	if (head) {
		ListNode<T>* temp = head;
		head = head->next;
		delete temp;
	}
	else {
		cout << "UnderFlow!" << endl;
	}
}

template<typename T>
void MySLL<T>::del_after(const T pos) {
	if (head == nullptr || head->next == nullptr) {
		cout << "UnderFlow!" << endl;
		return;
	}
	else {
		ListNode<T>* temp = head;

		while (temp->next) {
			if (temp->val == pos) {
				ListNode<T>* temp2 = temp->next;
				temp->next = temp2->next;
				delete temp2;
				return;
			}
			temp = temp->next;
		}
		cout << "Element Not Found" << endl;
	}
}

template<typename T>
void MySLL<T>::del_before(const T pos) {
	if (head == nullptr || head->next == nullptr || head->val == pos) {
		cout << "UnderFlow!" << endl;
	}
	else if (head->next->val == pos) {
		del();
	}
	else {
		ListNode<T>* temp = head->next;

		while (temp->next->next != nullptr){
			if (temp->next->next->val == pos) {
				ListNode<T>* temp2 = temp->next;
				temp->next = temp2->next;
				delete temp2;
				return;
			}
			temp = temp->next;
		}
		cout << "Element Not Found" << endl;
	}
}

template<typename T>
void MySLL<T>::display() {
	ListNode<T>* temp = head;

	cout << "Linked List";
	while (temp) {
		cout << " -> " << temp->val;
		temp = temp->next;
	}
	cout << endl;
}

template<typename T>
void menu() {
	int choice;
	T temp;
	MySLL<T> sll;
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
	menu<int>();
}