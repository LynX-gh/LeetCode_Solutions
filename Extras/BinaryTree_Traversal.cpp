#include<iostream>
#include<queue>

using namespace std;

template<typename T>
class TreeNode {
public:
	T val;
	TreeNode* left;
	TreeNode* right;
	TreeNode(const int x, TreeNode* ptr1, TreeNode* ptr2) : val(x), left(ptr1), right(ptr2) {}
};

template<typename T>
class MyBinaryTree {
public:
	TreeNode<T>* root;
	MyBinaryTree() : root(nullptr) { cout << "New Tree Created" << endl; }
	void insert(const T);
	void inorder(TreeNode<T>*);
	void preorder(TreeNode<T>*);
	void postorder(TreeNode<T>*);
};

template<typename T> void MyBinaryTree<T>::insert(const T x) {
	TreeNode<T>* n = new TreeNode<T>(x, nullptr, nullptr);

	if (root == nullptr) {
		root = n;
		return;
	}

	TreeNode<T> *temp = nullptr, *node = root;
	while (node != nullptr) {
		temp = node;
		if (x < node->val) {
			node = node->left;
		}
		else {
			node = node->right;
		}
	}
	if (x < temp->val) {
		temp->left = n;
	}
	else {
		temp->right = n;
	}
}

template<typename T> void MyBinaryTree<T>::inorder(TreeNode<T>* node) {
	if (node == nullptr) { return; }

	inorder(node->left);
	cout << node->val << "\t";
	inorder(node->right);
}

template<typename T> void MyBinaryTree<T>::preorder(TreeNode<T>* node) {
	if (node == nullptr) { return; }

	cout << node->val << "\t";
	preorder(node->left);
	preorder(node->right);
}

template<typename T> void MyBinaryTree<T>::postorder(TreeNode<T>* node) {
	if (node == nullptr) { return; }

	postorder(node->left);
	postorder(node->right);
	cout << node->val << "\t";
}

template<typename T> void menu() {
	int choice;
	T temp;
	MyBinaryTree<T> tree;
	while (true) {
		cout << "\n1. Insert\t\t2. InOrder\t\t3. PreOrder\t\t4. PostOrder\t\t5. Exit" << endl;
		cin >> choice;
		switch (choice)
		{
		case 1:
			cout << "Enter node value : ";
			cin >> temp;
			tree.insert(temp);
			break;
		case 2:
			tree.inorder(tree.root);
			cout << endl;
			break;
		case 3:
			tree.preorder(tree.root);
			cout << endl;
			break;
		case 4:
			tree.postorder(tree.root);
			cout << endl;
			break;
		case 5:
			exit(0);
		default:
			cout << "Enter a correct choice" << endl;
			cin.clear();
			cin.ignore(numeric_limits<streamsize>::max(), '\n');
		}
	}
}

int main() {
	menu<int>();
}