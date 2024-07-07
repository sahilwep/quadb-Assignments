#include<bits/stdc++.h>
using namespace std;

// Checking nth prime number
bool isPrime(int n){
    if(n == 1) return false;
    if(n == 2 || n == 3) return true;
    if(n % 2 == 0 || n % 3 == 0) return false;
    for(int i=5;i*i<=n;i = i+6){
        if(n % i == 0 || n % (i+2) == 0) return false;  // i & i + 2 combinations are the once that we have to check, after cancelling 2 & 3's multiples..
    }
    return true;
}

int main(){

    int n;
    cin >> n;
    (isPrime(n) == 1) ? cout << "Prime" : cout << "Not Prime";
    cout << endl;
    
    return 0;
}