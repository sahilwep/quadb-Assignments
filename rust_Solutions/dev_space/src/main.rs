use std::io::{self, BufRead};

fn max_subarray(arr: &[i32]) -> i32 {
    let mut res = arr[0];
    let mut max_ending = arr[0];
    
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