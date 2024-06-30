/*
// Question: Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number


// Observations: 
    * we are given a sorted array, & we need to find the first index of any given number let say 'k'
    * As the array is sorted, means we have elements in order..
    * as there is very less constrains given, let's assume array is sorted in ascending order..
    * elements can have duplicate values..
    * as we have no such constrains about 'k' range, let's say if the element are not in the range, we return -1, denoting that element is not found..

    * Example:
        I/p: a[] = {1, 2, 3, 4, 4, 5, 6}, k = 4
        O/p: 3(first occurrence)

        I/p: a[] = {1, 1, 1, 1, 1}, k = 1
        O/p: 3(first occurrence)

        I/p: a[] = {1, 1, 1, 1, 1}, k = 3
        O/p: -1(Element not found)

// Intrusion: 
    * As array is sorted, we can iterate over the array.
    * Whenever we hit that element we can return their index, else if element is not found, return '-1'

// Let assume Some Constrains:

    // Input Format: 
        * first line input of size of array, n.
        * second line input of 'k', element
        * third line input of array, let say constrains 1 <= arr[i] <= 10^5

    // Output Format: 
        * single line printing the index of first occurrence of element 'k',

// Example Testcases: 

// Input:

6 4
1 2 3 4 5 6
5 1
1 1 1 1 1
5 2
1 2 1 1 1

// Output:
3
0
1


// Time Complexity: 
    O(n) worse case when element itself found at last index.

*/

use std::io;

fn first_occurrence(arr: &Vec<i32>, k: i32) -> i32 {
    for (i, &item) in arr.iter().enumerate() {
        if item == k {
            return i as i32;
        }
    }
    -1
}

fn main() {
    let mut input = String::new();

    // Read the size of the array and the element to find
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input");
    let k: i32 = iter.next().unwrap().parse().expect("Invalid input");

    // Read the array elements
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input.split_whitespace()
                             .map(|x| x.parse().expect("Invalid input"))
                             .collect();

    // Print the first occurrence of the element
    println!("{}", first_occurrence(&arr, k));
}
