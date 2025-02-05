class Solution {
public:
    int minMutation(string startGene, string endGene, vector<string>& bank) {
        unordered_set<string> bank_set;
        unordered_set<string> visited;
        queue<pair<string, int>> q;

        q.push({startGene, 0});

        for (int i = 0; i < bank.size(); i++) {
            bank_set.insert(bank[i]);
        }
        bank_set.insert(startGene);

        while (!q.empty()) {
            auto [qGene, moves] = q.front();
            q.pop();

            if ( bank_set.contains(qGene) && !visited.contains(qGene) ) {
                if ( qGene == endGene ) {
                    return moves;
                }

                for (int i = 0; i < endGene.size(); i++) {
                    char temp = qGene[i];
                    if (temp != 'A') {
                        qGene[i] = 'A';
                        q.push({qGene, moves+1});
                    }
                    if (temp != 'C') {
                        qGene[i] = 'C';
                        q.push({qGene, moves+1});
                    }
                    if (temp != 'G') {
                        qGene[i] = 'G';
                        q.push({qGene, moves+1});
                    }
                    if (temp != 'T') {
                        qGene[i] = 'T';
                        q.push({qGene, moves+1});
                    }
                    qGene[i] = temp;
                }
            }

            visited.insert(qGene);
        }
        return -1;
    }
};