#include<iostream>
#include<vector>

using namespace std;

int binary_search(const vector<int>& arr, const int& x) {
	int low = 0, high = arr.size();
	while (low <= high) {
		int mid = floor((low + high) / 2);
		if (arr[mid] > x) { high = mid - 1; }
		else if (arr[mid] < x) { low = mid + 1; }
		else { return mid; }
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
	int res = binary_search(nums, num);

	if (res != -1) { cout << "The element is found in array at " << res << endl; }
	else { cout << "No such element is found in the array" << endl; }

	return 0;
}
