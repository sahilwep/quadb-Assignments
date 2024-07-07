#include<bits/stdc++.h>
using namespace std;

// function to find shortest word in string.
void shortestWord(string s){
    int minStartPos = -1; // store starting index of shortest word..
    int minSize = INT_MAX;    // store the size of shortest word.
    int currStart = -1;    // stores starting index of current word.
    int cnt = 0;    // counter to count each word size

    // iteration 0 to s.size() inclusive, case when we only have one word in string.
    for(int i=0;i<=s.size();i++){
        if(i < s.size() && s[i] != ' '){
            if(cnt == 0) currStart = i; // case to store starting index of every word, as before this iteration, flow goes to else when (s[i] == ' '), so we need to change new staring point...
            cnt++;  // increasing the counter
        }
        // condition when we encounter s[i] == ' '
        else{
            // condition when counter has count some character & it's lesser than the minSize that we found earlier..
            if(cnt > 0 && cnt < minSize){
                minSize = cnt;
                minStartPos = currStart; // storing the currStart into minStartPos, because we just encounter small size word, that was stored before...
            }
            // at last we will make counter  = 0
            cnt = 0;
        }
    }

    // now if there is no such strinf found, means there is empty string...
    if(minStartPos == -1){
        cout << "Empty String, no words found!" << endl;
    }else{
        // we have starting index of smallest word, & we know the size, we can iterate into this...
        for(int i=minStartPos;i<minStartPos+minSize;i++) cout << s[i];
        cout << endl;
    }
}

int main(){

    string s;
    getline(cin >> ws, s);  // input of string line, Usage of std::ws will extract all  the whitespace character.
    shortestWord(s);
    
    return 0;
}