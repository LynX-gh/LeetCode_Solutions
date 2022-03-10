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
	if(r)
}

void menu() {
	int max, choice, temp, temp2;
	cout << "Enter the max size of the stack : ";
	cin >> max;
	MyStack s(max);
	while (true) {
		cout << "\n1. Push\t\t2. Pop\t\t3. Peep\t\t4. Change\t\t5. Display\t\t6. Exit" << endl;
		cin >> choice;
		switch (choice)
		{
		case 1:
			cout << "Enter value to push : ";
			cin >> temp;
			s.push(temp);
			break;
		case 2:
			temp = s.pop();
			if (temp != -1) { cout << temp << endl; }
			break;
		case 3:
			cout << "Enter index to peep : ";
			cin >> temp;
			temp2 = s.peep(temp);
			if (temp2 != -1) { cout << temp2 << endl; }
			break;
		case 4:
			cout << "Enter index to change : ";
			cin >> temp;
			cout << "Enter new value : ";
			cin >> temp2;
			s.change(temp2, temp);
			break;
		case 5:
			s.display();
			break;
		case 6:
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
