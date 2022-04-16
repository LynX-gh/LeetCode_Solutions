#include<iostream>
#include<vector>
#include<unordered_set>
#include<queue>

using namespace std;

class MyGraph {
	int nodes;
	vector<vector<int>> adj_matrix;
	unordered_set<int> visited_dfs, visited_bfs;
	queue<int> breadth;
public:
	MyGraph(const int x) :nodes(x), adj_matrix(x, vector<int>(x, 0)) {}
	void input_edges();
	void show_adj_matrix();
	void dfs(const int);
	void bfs(const int);
};

void MyGraph::input_edges() {
	int input;
	for (int i = 0; i < nodes; i++) {
		for (int j = 0; j < nodes; j++) {
			cout << "Edge from " << i << " to " << j << " exists? (1/0) : ";
			cin >> input;
			input ? adj_matrix[i][j] = 1 : adj_matrix[i][j] = 0;
		}
	}
}

void MyGraph::show_adj_matrix() {
	cout << "Adjacent Matrix - " << endl;
	for (const vector<int> i : adj_matrix) {
		for (const int j : i) {
			cout << j << '\t';
		}
		cout << endl;
	}
}

void MyGraph::dfs(const int edge) {
	visited_dfs.insert(edge);
	cout << edge << '\t';
	for (int i = 0; i < nodes; i++) {
		if (adj_matrix[edge][i] && visited_dfs.find(i) == visited_dfs.end()) {
			dfs(i);
		}
	}
}

void MyGraph::bfs(const int edge) {

}

/*
void menu() {
	int choice, temp;
	cout << "Enter the number of nodes : ";
	cin >> temp;
	MyGraph graph(temp);
	while (true) {
		cout << "1. Create Edges\t\t2. DFS\t\t3. BFS\t\t4.Exit" << endl;
		cin >> choice;
		switch (choice)
		{
		case 1:
			graph.input_edges();
			break;
		case 2:
			graph.dfs();
			break;
		case 3:
			graph.bfs();
			break;
		case 4:
			exit(0);
		default:
			cout << "Enter a valid choice : " << endl;
			cin.clear();
			cin.ignore(numeric_limits<streamsize>::max(), '\n');
		}
	}
}
*/

int main() {
	int temp;
	cout << "Enter the number of nodes : ";
	cin >> temp;
	MyGraph graph(temp);
	graph.input_edges();
	system("cls");
	graph.show_adj_matrix();
	cout << "\nEnter Starting Node : ";
	cin >> temp;
	cout << "\nDFS Traversal - " << endl;
	graph.dfs(temp);
	cout << "\nBFS Traversal - " << endl;
	graph.bfs(temp);
}
