#include<iostream>
#include<vector>

using namespace std;

class MyStack {
private:
	vector<int> Stack;
	int top, max;

public:
	MyStack(const int m) :top(-1), max(m), Stack(m, 0) { cout << "New Stack is created" << endl; }
	void push(const int x);
	int pop();
	int peep(const int i);
	void change(const int x, const int i);
	void display();
};

void MyStack::push(const int x) {
	if (top+1 < max) {
		Stack[++top] = x;
	}
	else {
		cout << "OverFlow!" << endl;
	}
}

int MyStack::pop() {
	if (top != -1) {
		return Stack[top--];
	}
	else {
		cout << "UnderFlow!" << endl;
		return -1;
	}
}

int MyStack::peep(const int i) {
	if (top - i + 1 >= 0) {
		return Stack[i - 1];
	}
	else {
		cout << "UnderFlow!" << endl;
		return -1;
	}
}

void MyStack::change(const int x, const int i) {
	if (top - i + 1 >= 0) {
		Stack[i - 1] = x;
	}
	else {
		cout << "UnderFlow!" << endl;
	}
}

void MyStack::display() {
	for (int i = 0; i <= top; i++) {
		cout << Stack[i] << '\t';
	}
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
			cout << endl;
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
