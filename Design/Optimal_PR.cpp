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

void optimal_remove(const vector<int>& pr, list<int>& frame, const int pos) {
	int max_page, max_next_use = 0;
	for (const int page : frame) {
		int next_use = 0;
		for (int j = pos; j < pr.size(); j++) {
			if (page == pr[j]) {
				next_use = j;
				break;
			}
		}
		if (next_use = 0) {
			frame.remove(page);
			return;
		}
		else {
			if (next_use > max_next_use) {
				max_next_use = next_use;
				max_page = page;
			}
		}
	}
	frame.remove(max_page);
}

int optimal(const vector<int>& pr, const int fsize) {
	list<int> frame;
	int faults = 0;
	cout << "Optimal Results - ";

	for (int i = 0; i < pr.size(); i++) {
		if (hit_or_miss(pr[i], frame)) {
			cout << "\tH";
		}
		else {
			if (frame.size() > fsize) {
				optimal_remove(pr, frame, i);
			}
			frame.push_back(pr[i]);
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

	int faults = optimal(page_ref, frame_size);
	cout << "\n\nMiss Ratio - " << float(faults) / pr_size;

	return 0;
}