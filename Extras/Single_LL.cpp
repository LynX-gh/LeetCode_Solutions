#include<iostream>

using namespace std;

template<class T>
class ListNode {
public:
	T val;
	ListNode* next;
	ListNode(T v, ListNode* ptr) :val(v), next(ptr) {}
};

template<class T>
class MySLL {
	ListNode<T>* head;
public:
	MySLL() :head(nullptr) {}
	~MySLL();
	void insert(const T);
	void insend(const T);
	void insord(const T);
	void del();
	void del_after(const T);
	void del_before(const T);
	template<typename U> friend ostream& operator<<(ostream&, const MySLL<U>&);
};

template<typename T>
MySLL<T>::~MySLL() {
	ListNode* temp;
	while (head) {
		temp = head;
		head = head->next;
		delete head;
	}
}

template<class T> void MySLL<T>::insert(const T x) {
	ListNode<T>* n = new ListNode<T>(x, head);

	head = n;
}

template<class T> void MySLL<T>::insend(const T x) {
	ListNode<T>* n = new ListNode<T>(x, nullptr);

	if (head == nullptr) {
		head = n;
		return;
	}

	ListNode<T>* temp = head;
	while (temp->next) {
		temp = temp->next;
	}
	temp->next = n;
}

template<class T> void MySLL<T>::insord(const T x) {
	if (head == nullptr || head->val > x) {
		insert(x);
		return;
	}

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

template<class T> void MySLL<T>::del() {
	if (head == nullptr) {
		cout << "UnderFlow!" << endl;
		return;
	}

	ListNode<T>* temp = head;
	head = head->next;
	delete temp;
}

template<class T> void MySLL<T>::del_after(const T pos) {
	if (head == nullptr || head->next == nullptr) {
		cout << "UnderFlow!" << endl;
		return;
	}

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

template<class T> void MySLL<T>::del_before(const T pos) {
	if (head == nullptr || head->next == nullptr || head->val == pos) {
		cout << "UnderFlow!" << endl;
		return;
	}
	if (head->next->val == pos) {
		del();
		return;
	}

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

template<class U> ostream& operator<<(ostream& os, const MySLL<U>& sll) {
	ListNode<U>* temp = sll.head;

	os << "Linked List";
	while (temp) {
		os << " -> " << temp->val;
		temp = temp->next;
	}
	os << endl;
	return os;
}

template<class T> void menu() {
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
			cout << sll;
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