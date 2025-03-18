#include<iostream>
#include<vector>

using namespace std;

int linear_search(const vector<int>& arr, const int& x) {
	for (int i = 0; i < arr.size(); i++) {
		if (arr[i] == x)
			return i+1;
	}
	return -1;
}

int main() {
	int size = 0, num = 0;
	vector<int> nums;

	cout << "Enter the number of elements in array : ";
	cin >> size;

	for (int i = 0; i < size; i++) {
		cout << "Enter the next number : ";
		cin >> num;
		nums.push_back(num);
	}

	cout << "Enter the number to search for : ";
	cin >> num;
	int res = linear_search(nums, num);

	if (res != -1) { cout << "The element is found in array at position " << res << endl; }
	else { cout << "No such element is found in the array" << endl; }

	return 0;
}
