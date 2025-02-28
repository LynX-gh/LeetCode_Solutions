#include<iostream>
#include<vector>
#include<list>

using namespace std;

bool hit_or_miss(const int x, const list<int>& frame) {
	for (const int page : frame) {
		if (page == x)
			return true;
	}
	return false;
}

int fifo(const vector<int>& pr, const int fsize) {
	list<int> frame;
	int faults = 0;
	cout << "FIFO Results - ";

	for (const int process : pr) {
		if (hit_or_miss(process, frame)) {
			cout << "\tH";
		}
		else {
			if (frame.size() > fsize) {
				frame.pop_front();
			}
			frame.push_back(process);
			cout << "\tM";
			faults++;
		}
	}
	cout << endl;
	return faults;
}

int main() {
	vector<int> page_ref;
	int pr_size, frame_size, temp;

	cout << "Enter number of frames : ";
	cin >> frame_size;
	cout << "Enter number of processes : ";
	cin >> pr_size;
	cout << "Enter the processes : " << endl;
	for (int i = 0; i < pr_size; i++) {
		cin >> temp;
		page_ref.push_back(temp);
	}

	int faults = fifo(page_ref, frame_size);
	cout << "\n\nMiss Ratio - " << float(faults) / pr_size;

	return 0;
}