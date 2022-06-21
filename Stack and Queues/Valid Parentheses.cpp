class Solution {
public:
    bool isValid(string str) {
        stack<char> s;
        for(int i = 0; i < str.length(); i++){
            char c = str[i];
            if(c == '{' || c == '[' || c == '('){
                s.push(c);
            }
            else if(s.empty()){
                return false;
            }
            else{
                char t = s.top();
                if(t == '{' && c == '}')
                    s.pop();
                else if(t == '[' && c == ']')
                    s.pop();
                else if(t == '(' && c == ')')
                    s.pop();
                else
                    return false;
            }
        }
        return s.empty();
    }
};