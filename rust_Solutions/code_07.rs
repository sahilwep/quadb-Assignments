/*
//  Question: Implement a function that returns the kth smallest element in a given array.

// Observations: 
    * We are given an array, & we have to return the kth smallest element from that array..

// Intrusion: 
    * First we will sort an array, & then from there we will return the kth smallest digit..
    * We will maintain counter that will count unique element from the sorted array & once we have kth = count we return that element


// Example Testcases:
// Input:
6 3 
7 10 4 3 20 15
5 4
7 10 4 20 15

// Ouput:
7
15

// Time Complexity:
    O(nlogn)

*/

use std::io;

fn kth_element(arr: &mut [i32], k: usize) -> i32 {
    arr.sort();
    let mut cnt = 1;
    for i in 1..arr.len() {
        if arr[i] != arr[i - 1] {
            cnt += 1;
        }
        if k == cnt {
            return arr[i];
        }
    }
    -1 // no such element or all elements are the same
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input");
    let k: usize = iter.next().unwrap().parse().expect("Invalid input");

    let mut arr = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    for num in input.split_whitespace() {
        let num: i32 = num.parse().expect("Invalid input");
        arr.push(num);
    }

    if arr.len() != n {
        println!("Invalid input");
        return;
    }

    let result = kth_element(&mut arr, k);
    println!("{}", result);
}



