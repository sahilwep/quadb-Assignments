#include<bits/stdc++.h>
using namespace std;

// Palindrome Checking function:
bool isPalindrome(string s){
    int start = 0, end = s.size()-1;
    while(start <= end){
        if(s[start++] != s[end--]) return 0;
    }
    return 1;
}

int main(){
    string s;
    cin >> s;
    (isPalindrome(s) == 1) ? cout << "Palindrome" : cout << "Not Palindrome";
    cout << endl;

    return 0;
}