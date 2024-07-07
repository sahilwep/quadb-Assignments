#include<bits/stdc++.h>
using namespace std;

// Maximum subarray Sum:
int maxSubarray(int arr[], int n){
    int res = arr[0];   // initialize our res = arr[0], let say default max.
    int maxEnding = arr[0]; // maxEnding is arr[0], because there is nothing before it.
    // iterating from 1 --to--> n-1
    for(int i=1;i<n;i++){
        maxEnding = max((maxEnding + arr[i]), arr[i]);  // getting max from (maxEnding + arr[i]) or arr[i], whatever the max value, we take it as our maxEnding
        res = max(res, maxEnding);  // compare res with maxEnding & store in res. 
    }
    return res;
}

int main(){
    int n;
    cin >> n;
    int arr[n];
    for(int i=0;i<n;i++) cin >> arr[i];

    cout << maxSubarray(arr, n) << endl;
    
    return 0;
}