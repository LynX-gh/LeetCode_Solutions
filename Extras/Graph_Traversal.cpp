#include<iostream>
#include<vector>
#include<unordered_set>
#include<queue>

using namespace std;

class MyGraph {
	int nodes;
	vector<vector<bool>> adjMatrix;
	unordered_set<int> visitedDfs, visitedBfs;
public:
	MyGraph(const int x) :nodes(x), adjMatrix(x, vector<bool>(x)) {}
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
			input ? adjMatrix[i][j] = true : adjMatrix[i][j] = false;
		}
	}
}

void MyGraph::show_adj_matrix() {
	cout << "Adjacent Matrix - " << endl;
	for (const vector<bool> i : adjMatrix) {
		for (const bool j : i) {
			cout << j << '\t';
		}
		cout << endl;
	}
}

void MyGraph::dfs(const int start) {
	visitedDfs.insert(start);
	cout << start << '\t';
	for (int i = 0; i < nodes; i++) {
		if (adjMatrix[start][i] && visitedDfs.find(i) == visitedDfs.end()) {
			dfs(i);
		}
	}
}

void MyGraph::bfs(const int start) {
	queue<int> next;
	next.push(start);
	while (!next.empty()) {
		int curEdge = next.front();
		next.pop();
		if (visitedBfs.find(curEdge) == visitedBfs.end()) {
			visitedBfs.insert(curEdge);
			cout << curEdge << "\t";
			for (int i = 0; i < nodes; i++) {
				if (adjMatrix[curEdge][i] && (visitedBfs.find(i) == visitedBfs.end())) {
					next.push(i);
				}
			}
		}
	}
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
	cout << "\n\nBFS Traversal - " << endl;
	graph.bfs(temp);
}
