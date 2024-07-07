#include<bits/stdc++.h>
using namespace std;

// function to check first occurrence of element 'k'
int firstOccurrence(vector<int> arr, int k){
    for(int i=0;i<arr.size();i++){
        if(arr[i] == k) return i;   // returning the first occurrence of element k
    }
    return -1;  // else we did'nt find the first occurrence of that element..
}

int main(){
    int n, k;   // n = size of array, k = element that we want to find..
    cin >> n >> k;  // input
    vector<int>  arr(n);  // declaring vector of size n
    for(int i=0;i<n;i++) cin >> arr[i]; // input of array

    cout << firstOccurrence(arr, k) << endl;

    return 0;
}