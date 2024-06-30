/*
//  Question: Reverse a string in Rust

// Observations: 
    * We need to reverse a string
    * Exapmle:
        sahilwep    -> pewlihas
        boss    -> ssob   

// Intrusion: 
    * We can use two pointer approach: 
        * first pointer will be at starting index..
        * second pointer will be at last index...
        * we swap these pointer locations characters..
        * we increment first pointer, & decrement second pointer...
        * once we reach till n/2, we break out...


// Exapmle Testcases:
// Input: 
sahilwep
sahil
boss

// Output:
pewlihas
lihas
ssob


// Time complexity; 
    O(N/2) -> O(N)

*/

use std::io;

// Function to reverse a mutable string
fn reverse_string(s: &mut String) {
    let mut start = 0;
    let mut end = s.len() - 1;
    let s_bytes = unsafe { s.as_bytes_mut() };

    while start < end {
        // Swap characters at indices start and end
        s_bytes.swap(start, end);   // swap, these two pointed characters..

        // move start pointer forward & end pointer backward
        start += 1;
        end -= 1;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut s = input.trim().to_string();
    
    reverse_string(&mut s);
    println!("{}", s);
}

