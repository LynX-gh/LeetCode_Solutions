#include <iostream>

using namespace std;

class LinkNode{
	public:
		int val;
		LinkNode* next;
		LinkNode(int x, LinkNode* ptr): val(x), next(ptr){}
};

class SLL{
	private:
		LinkNode* head;
	public:
		SLL(): head(nullptr){}
		void insert(const int x);
		void del();
		int top();
};

void SLL::insert(const int x){
	LinkNode* n = new LinkNode(x, head);
	head = n;
}

void SLL::del(){
	if(head){
		LinkNode* tmp = head;
		head = head->next;
		delete tmp;
		cout<<"Success!"<<endl;
	}
	else{
		cout<<"UnderFlow!"<<endl;
	}
}

int SLL::top(){
	if(head){
		return head->val;
	}
	else{
		return -1;
	}
}

void menu(){
	int choice, temp;
	SLL s;
	while(true){
		cout<<"1. Insert\t\t2. Delete\t\t3. Top\t\t4. Exit"<<endl;
		cin>>choice;
		switch(choice){
			case 1:
				cout<<"Enter value to insert : ";
				cin>>temp;
				s.insert(temp);
				break;
			case 2:
				s.del();
				break;
			case 3:
				temp = s.top();
				if(temp == -1){cout<<"UnderFlow!"<<endl;}
				else{cout<<"Value of top : "<<temp<<endl;}
				break;
			case 4:
				exit(0);
			default:
				cout<<"Enter a correct choice"<<endl;
				cin.clear();
				cin.ignore(numeric_limits<streamsize>::max(), '\n');
		}
	}
}

int main(){
	menu();
}