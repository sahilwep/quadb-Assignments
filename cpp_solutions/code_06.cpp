#include<bits/stdc++.h>
using namespace std;

// Function to find the largest common prefix:
string longestCommonPrefix(vector<string> s){
    if (s.empty()) return "-1";   // when there is no words in string we return .

    sort(s.begin(), s.end()); // Sort the array of strings

    // after sorting:
    string first = s[0];    // first index string
    string last = s[s.size() - 1];  // last index string
    int minLength = min(first.size(), last.size());     // as we have to iterate over these two string, we need the minimum size among these two..

    // Find the common prefix between the first and last string
    int i = 0;
    string prefix = "";    // making res string that will store the prefix..
    while (i < minLength && first[i] == last[i]) {
        prefix += first[i];    // storing the prefix..
        i++;
    }

    return prefix; // last we return the prefix string.
}

int main(){
    int n;
    cin >> n;
    vector<string> s(n);
    for(int i=0;i<n;i++){
        cin >> s[i];
    }

    cout << longestCommonPrefix(s) << endl;

    return 0;
}