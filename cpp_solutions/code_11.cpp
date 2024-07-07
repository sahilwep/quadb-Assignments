#include<bits/stdc++.h>
using namespace std;

// Merge two sorted array:
void merge(int a[], int b[], int n, int m){
    int i=0,j=0;    // using two pointers that help to store array in sorted order in third array.
    int arr[n+m];   // initialization of third array of size (n+m), which is of both the array size..
    int k = 0;  // pointer for third array that will store element from both the array..
    // when pointer i < n & j < m, then only it goes into the scope..
    while(i<n && j<m){
        // condition when first array element is smaller than the second array element..
        if(a[i] < b[j]){
            arr[k++] = a[i++];  // storing the element & increment the pointer..
        }
        // condition when second array element is lesser than first array element..
        else{
            arr[k++] = b[j++];  // storing & incrementing the pointers..
        }
    }
    // for left over in first array...
    while(i<n){
        arr[k++] = a[i++];  // storing the first array element to the final array..
    }
    // for left over in second array...
    while(j<m){
        arr[k++] = b[j++];  // storing the second array element to the final array..
    }

    // last we print the third array..
    for(int i = 0;i<m+n;i++){
        cout << arr[i] << " ";
    }
    cout << endl;
}


int main(){
    int n, m;
    cin >> n >> m;
    int a[n];
    int b[m];
    for(int i=0;i<n;i++) cin >> a[i];
    for(int i=0;i<m;i++) cin >> b[i];
	
	merge(a,b,n,m);

    return 0;
}