#include<iostream>
#include<vector>

using namespace std;

vector<int> merge(vector<int> larr, vector<int> rarr) {
    vector<int> result(larr.size()+rarr.size());
    unsigned i = 0;
    unsigned j = 0;
    unsigned k = 0;

    while (i < larr.size() && j < rarr.size()) {
        if (larr[i] < rarr[j]) {
            result[k] = larr[i];
            i++;
        }
        else {
            result[k] = rarr[j];
            j++;
        }
        k++;
    }

    while (i < larr.size()) {
        result[k] = larr[i];
        i++;
        k++;
    }

    while (j < rarr.size()) {
        result[k] = rarr[j];
        j++;
        k++;
    }

    return result;
}

vector<int> merge_sort(vector<int> arr) {
    if (arr.size() > 1) {
        int mid = arr.size() / 2;
        vector<int> lefthalf(arr.begin(), arr.begin() + mid);
        vector<int> righthalf(arr.begin() + mid, arr.begin() + arr.size());
        lefthalf = merge_sort(lefthalf);
        righthalf = merge_sort(righthalf);
        arr = merge(lefthalf, righthalf);
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

	nums = merge_sort(nums);
	cout << "The sorted array is : " << endl;
	for (const int i : nums) { cout << i << "\t"; }

	return 0;
}
