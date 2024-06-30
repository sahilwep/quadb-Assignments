/*
//  Question: Given a sorted array of integers, implement a function that returns the median of the array.

// Observations: 
    * As array is already sorted, we have to return median of the array.
    * Median: 
        * middle element..
        * If the array size is odd, we simply return (n/2)th index element..
        * If the array size is even, we have to get the sum of (n/2-1)th, (n/2+1)th element sum & divide it by 2, & return the result..


// Intrusion: 
    * we just have to check whether the array size is odd or even, & then we can do the following operations that we have discussed earlier in observations..

// Input format: 
    * We will first take size of an array, & then we will take input of array elements.

// Exapmle Testcases:
// Input: 
4
1 2 3 4
5
1 2 3 4 5
4
1 1 1 1
// Output: 
2.5
3
1


// Time Complexity: 
    O(1)

*/

use std::io;

fn find_median(mut arr: Vec<i32>) {
    arr.sort();
    let n = arr.len();
    if n % 2 != 0 {
        println!("{}", arr[n / 2]);
    } else {
        let res = (arr[(n / 2) - 1] + arr[n / 2]) as f64 / 2.0;
        println!("{}", res);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut arr = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    for num in input.split_whitespace() {
        let num: i32 = num.parse().expect("Invalid input");
        arr.push(num);
    }

    find_median(arr);
}

