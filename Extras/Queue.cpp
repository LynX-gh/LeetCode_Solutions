#include<iostream>
#include<vector>

using namespace std;

class MyQueue {
private:
	vector<int> Queue;
	int f, r, max;

public:
	MyQueue(const int m) :max(m), f(-1), r(-1), Queue(m, 0) { cout << "New Queue is created"; }
	void enq(const int x);
	int deq();
	void display();
};

void MyQueue::enq(const int x) {
	if (r + 1 < max) {
		r++;
		Queue[r] = x;
		if (f == -1) { f++; }
	}
	else {
		cout << "OverFlow!" << endl;
	}
}

int MyQueue::deq() {
	if (f != r) {
		f++;
		return Queue[f-1];
	}
	else if (f == 0) {
		f = r = -1;
		return Queue[0];
	}
	else {
		cout << "UnderFlow!" << endl;
		return -1;
	}
}

void MyQueue::display() {
	for (int i = f; i <= r; i++) {
		cout << Queue[i] << '\t';
	}
	cout << endl;
}

void menu() {
	int max, choice, temp;
	cout << "Enter the max size of the queue : ";
	cin >> max;
	MyQueue s(max);
	while (true) {
		cout << "\n1. EnQueue\t\t2. DeQueue\t\t3. Display\t\t4. Exit" << endl;
		cin >> choice;
		switch (choice)
		{
		case 1:
			cout << "Enter value to push : ";
			cin >> temp;
			s.enq(temp);
			break;
		case 2:
			temp = s.deq();
			if (temp != -1) { cout << temp << endl; }
			break;
		case 3:
			s.display();
			break;
		case 4:
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
