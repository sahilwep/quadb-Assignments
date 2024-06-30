/*
// Question: Implement a function that checks whether a given string is a palindrome or not.

// Observations: 
    * For string to be palindrome, we need to check these conditions:
        * Let say we have string s, of size n.
        * we need to check string characters before "n/2", and after "n/2"...
        * If these character are same throughout traversing.. we can say it's palindrome : else "Not Palindrome"
        * Single characters will always be palindrome...

// Intrusion: 
    * We can use two pointers..
        * First pointer will be at index '0', say start
        * second pointer will be at last 'n-1' say end
        * we can iterate start towards "n/2" & same end towards "n/2", if these characters are not same, we can return false else return true..


// Example Test Cases: 

// Input: 
sahilwep
aabaa
aaaa
abab
baab

// Ouput: 
Not Palindrome
Palindrome
Palindrome
Not Palindrome
Palindrome



// Time Complexity: 
    O(N/2) -> O(N)

*/

use std::io::{self, Write};

fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut end = s.len() - 1;
    
    while start <= end {
        if bytes[start] != bytes[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim();

    if is_palindrome(s) {
        println!("Palindrome");
    } else {
        println!("Not Palindrome");
    }
}
