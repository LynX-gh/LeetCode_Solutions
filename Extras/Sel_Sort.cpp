#include<iostream>
#include<vector>

using namespace std;

vector<int> sel_sort(vector<int>& arr) {
	int min;
	for (int i = 0; i < arr.size() - 1; i++) {
		min = i;
		for (int j = i + 1; j < arr.size(); j++) {
			if (arr[min] > arr[j]) { min = j; }
		}
		int temp = arr[i];
		arr[i] = arr[min];
		arr[min] = temp;
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

	sel_sort(nums);
	cout << "The sorted array is : " << endl;
	for (const int i : nums) {
		cout << i << "\t";
	}

	return 0;
}
