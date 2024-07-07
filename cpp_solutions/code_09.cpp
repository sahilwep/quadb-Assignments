#include<bits/stdc++.h>
using namespace std;

// reverse string:
void reverseString(string &s){
    int start = 0;
    int end = s.size()-1;
    while(start <= end) swap(s[start++], s[end--]);
}

int main(){
    string s;
    cin >> s;
    reverseString(s);
    cout << s << endl;

    return 0;
}