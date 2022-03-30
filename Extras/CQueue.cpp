#include<iostream>
#include<vector>

using namespace std;

class MyCQueue {
private:
	vector<int> Queue;
	int f, r, max;

public:
	MyCQueue(const int m) :max(m), f(-1), r(-1), Queue(m, 0) { cout << "New Queue is created"; }
	void cenq(const int x);
	int cdeq();
	void display();
};

void MyCQueue::cenq(const int x) {
	if (f == -1) {
		f = r = 0;
		Queue[r] = x;
	}
	else if (r + 1 == max) {
		if (f != 0) {
			r = 0;
			Queue[r] = x;
		}
		else {
			cout << "OverFlow!" << endl;
		}
	}
	else if (r + 1 == f) {
		cout << "OverFlow!" << endl;
	}
	else {
		Queue[++r] = x;
	}
}

int MyCQueue::cdeq() {
	if (f != -1) {
		int temp = Queue[f];
		if (r < f) {
			if (f + 1 < max) {
				f++;
			}
			else {
				f = 0;
			}
		}
		else if (r > f) {
			f++;
		}
		else {
			f = r = -1;
		}
		return temp;
	}
	else {
		cout << "UnderFlow!" << endl;
		return -1;
	}
}

void MyCQueue::display() {
	if (r < f) {
		for (int i = f; i < max; i++) {
			cout << Queue[i] << '\t';
		}
		for (int i = 0; i <= r; i++) {
			cout << Queue[i] << '\t';
		}
		cout << endl;
	}
	else {
		for (int i = f; i <= r; i++) {
			cout << Queue[i] << '\t';
		}
		cout << endl;
	}
}

void menu() {
	int max, choice, temp;
	cout << "Enter the max size of the queue : ";
	cin >> max;
	MyCQueue s(max);
	while (true) {
		cout << "\n1. EnQueue\t\t2. DeQueue\t\t3. Display\t\t4. Exit" << endl;
		cin >> choice;
		switch (choice)
		{
		case 1:
			cout << "Enter value to push : ";
			cin >> temp;
			s.cenq(temp);
			break;
		case 2:
			temp = s.cdeq();
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
