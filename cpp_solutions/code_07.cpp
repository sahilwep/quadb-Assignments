#include<bits/stdc++.h>
using namespace std;

int kthElement(int arr[], int n, int k){
    sort(arr, arr+n);
    int cnt = 1;
    for(int i=1;i<n;i++){
        if(arr[i] != arr[i+1]) cnt++;
        if(k == cnt){
            return arr[i];
        }
    }
    return -1;  // no such element or all elements are same..
}

int main(){
    int n, k;
    cin >> n >> k;
    int arr[n];
    for(int i=0;i<n;i++) cin >> arr[i];

    cout << kthElement(arr, n, k) << endl;

    return 0;
}