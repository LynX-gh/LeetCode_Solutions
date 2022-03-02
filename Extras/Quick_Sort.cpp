#include<iostream>
#include<vector>

using namespace std;

int partition(vector<int>& arr, const int start, const int end) {
	int partition = start;
	for (int i = start; i < end; i++) {
		if (arr[i] < arr[end]) {
			swap(arr[i], arr[partition]);
			partition++;
		}
	}
	swap(arr[end], arr[partition]);
	return partition;
}

vector<int> quick_sort(vector<int>& arr, const int start, const int end) {
	if (start < end) {
		int pivot = partition(arr, start, end);
		quick_sort(arr, start, pivot - 1);
		quick_sort(arr, pivot + 1, end);
	}
	return arr;
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

	quick_sort(nums, 0, size-1);
	cout << "The sorted array is : " << endl;
	for (const int i : nums) { cout << i << "\t"; }

	return 0;
}
