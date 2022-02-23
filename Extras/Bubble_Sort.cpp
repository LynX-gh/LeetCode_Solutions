#include<iostream>
#include<vector>

using namespace std;

vector<int> bubble_sort(vector<int>& arr) {
	int ctr = arr.size()-1, flag = 0;
	for (int i = 0; i < arr.size()-1; i++) {
		for (int j = 0; j < ctr; j++) {
			if (arr[j] > arr[j + 1]) {
				int temp = arr[j];
				arr[j] = arr[j + 1];
				arr[j + 1] = temp;
				flag++;
			}
		}
		if (flag == 0) { return arr; }
		flag = 0;
		ctr--;
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

	bubble_sort(nums);
	cout << "The sorted array is : " << endl;
	for (const int i : nums) {
		cout << i << "\t";
	}

	return 0;
}
