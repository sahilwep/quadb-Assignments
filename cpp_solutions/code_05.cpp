#include<bits/stdc++.h>
using namespace std;

// Median of an array
void findMedian(vector<int> arr){
    int n = arr.size();
    // when size is odd
    if(n % 2 != 0){
        cout << arr[n/2] << endl;
    }
    else{
        double res = double(arr[(n/2) - 1] + arr[n/2]) / 2;
        cout << res << endl;
    }
}

int main(){

    int n;
    cin >> n;
    vector<int> arr(n);
    for(int i=0;i<n;i++) cin >> arr[i];
    findMedian(arr);

    return 0;
}