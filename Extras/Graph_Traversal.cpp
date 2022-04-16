#include<iostream>
#include<vector>
#include<unordered_set>
#include<queue>

using namespace std;

class MyGraph {
	int nodes;
	vector<vector<bool>> adj_matrix;
	unordered_set<int> visited;
public:
	MyGraph(const int x) :nodes(x), adj_matrix(x, vector<bool>(x)) {}
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
			input ? adj_matrix[i][j] = true : adj_matrix[i][j] = false;
		}
	}
}

void MyGraph::show_adj_matrix() {
	cout << "\n\nAdjacent Matrix - " << endl;
	for (const vector<bool> i : adj_matrix) {
		for (const bool j : i) {
			cout << j << '\t';
		}
		cout << endl;
	}
}

void MyGraph::dfs(const int start) {
	visited.insert(start);
	cout << start << '\t';
	for (int i = 0; i < nodes; i++) {
		if (adj_matrix[start][i] && visited.find(i) == visited.end()) {
			dfs(i);
		}
	}
}

void MyGraph::bfs(const int start) {
	queue<int> next;

	visited.clear();
	next.push(start);
	while (!next.empty()) {
		int curr_node = next.front();
		next.pop();
		if (visited.find(curr_node) == visited.end()) {
			visited.insert(curr_node);
			cout << curr_node << "\t";
			for (int i = 0; i < nodes; i++) {
				if (adj_matrix[curr_node][i] && (visited.find(i) == visited.end())) {
					next.push(i);
				}
			}
		}
	}
}

int main() {
	int temp;
	cout << "Enter the number of nodes : ";
	cin >> temp;
	MyGraph graph(temp);
	graph.input_edges();
	graph.show_adj_matrix();
	cout << "\nEnter Starting Node : ";
	cin >> temp;
	cout << "\nDFS Traversal -    ";
	graph.dfs(temp);
	cout << "\nBFS Traversal -    ";
	graph.bfs(temp);
	cout << endl;
}
