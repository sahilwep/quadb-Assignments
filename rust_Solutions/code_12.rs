/*
//  Question: Find the maximum subarray sum in Rust


// Observations: 
    * We have to find the element's of a contiguous subarray of numbers that has the largest sum.

// What is subarray?
    subarray of {1, 2, 3}
    : {1}, {2}, {3}, {1, 2}, {2, 3}, {1, 2, 3}
    Note : {1, 3} are not subarray of contiguous element.

// Example:
    I/p: arr[] = {2, 3, -8, 7, -1, 2, 3}
    O/p: 11

    I/p: arr[] = {5, 8, 3}
    O/p: 16

    I/p: arr[] = {-6, -1, -8}
    O/p: -1


// Intrusion: 
    * We traverse the array from left to right:
        * For every element we find out the maximum sum of sub array that must end with this element.
        * Our overall result is going to be maximum of all these values.

        arr:              { -5,  1,  -2,  3,  -1,  2, -2 } 
                            |   |    |   |    |    |   |
        maxEnding          -5   1   -1   3    2    4   2
                                                   |
                                                 Result
    * Idea is to extend the sub-array or start a new subarray:
    * We can use maxEnding of prev element + current element to extend the sub array,
    * or we can use current element to start a new subarray.
    * Using max(), with these two we can take the max value.
            MaxEnding[i] = max((maxEnding[i-1] + arr[i]), arr[i])


// Example Testcases:
// Input:
7
2 3 -8 7 -1 2 3
3
5 8 3
3
-6 -1 -8

// Output: 
11
16
-1

// Time Complexity:
     O(n)

*/

use std::io::{self, BufRead};

// Maximum subarray Sum
fn max_subarray(arr: &[i32]) -> i32 {
    let mut res = arr[0];       // Initialize result with first element
    let mut max_ending = arr[0]; // Initialize maxEnding with first element
    
    for &num in &arr[1..] {
        max_ending = std::cmp::max(max_ending + num, num); // Calculate maxEnding for current element
        res = std::cmp::max(res, max_ending); // Update result if maxEnding is greater
    }
    
    res
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|line| line.unwrap());

    let n: usize = iterator.next().unwrap().trim().parse().unwrap(); // Read input size
    let arr: Vec<i32> = iterator
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect(); // Read input array

    let result = max_subarray(&arr);
    println!("{}", result);
}
