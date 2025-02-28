#include<iostream>
#include<vector>
#include<tuple>

using namespace std;

class Process{
public:
    int at, bt, ft, tat, wt;
    
    Process(int x, int y) :at(x), bt(y), ft(0), tat(0), wt(0) {}
    
    void complete(int x){
        ft = x;
        tat = ft-at;
        wt = tat-bt;
    }
};

void sort_process(vector<Process>& p){
    int min = 0;
    for(unsigned int i = 0; i < p.size(); i++){
        min = i;
        for(unsigned int j = i+1; j < p.size(); j++){
            if(p[min].at > p[j].at){
                min = j;
            }
        }
		swap(p[min], p[i]);
    }
}

pair<float, float> fcfs(vector<Process>& p) {
    int clock_time = 0;
    float avgwt = 0, avgtat = 0;
    for (Process& elem : p) {
        if (elem.at > clock_time) { clock_time += elem.at - clock_time; }
        clock_time += elem.bt;
        elem.complete(clock_time);
        avgwt += elem.wt;
        avgtat += elem.tat;
        cout << elem.at << '\t' << elem.bt << '\t' << elem.ft << '\t' << elem.tat << '\t' << elem.wt << endl;
    }
    return make_pair(avgtat, avgwt);
}

int main()
{
    vector<Process> processes;
    int n, at, bt;

    cout<<"Enter the number of processes to run :";
    cin>>n;
	
    for(int i = 0; i<n; i++){
        cout<<"Enter the arrival time of process "<<i<<" :";
        cin>>at;
        cout<<"Enter the burst time of process "<<i<<" :";
        cin>>bt;
        processes.push_back(Process(at, bt));
    }
	
    sort_process(processes);
    auto [avgtat, avgwt] = fcfs(processes);

    cout<<"\nAverage Waiting Time :"<<avgwt/n<<"\nAverage TAT : "<<avgtat/n<<endl;
}