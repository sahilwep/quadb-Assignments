/*
// Question:  Given a string of words, implement a function that returns the shortest word in the string.

// Observations: 
    * We are given a string that contains words, means there should be ' ', empty character b/w long string..
    * We need to return the shortest word, means shortest contigious character in string.
    * It's possible that many elements have equal length..
        * In that case we will return the first occurrence, because there is no such constrains given.. 
    * There might be a possibility that string contains only one element...
        * In that case we need to check this explicitly or 
        * instead we can iterate from 0 to s.size() (inclusive), that will check for single word in string..
        * & we can use condition i.e (i < s.size()), while counting when we have multiple words..

// Intrusion: 
    * Idea is to iterate over the string, & whenever we encounter ' ' empty character, we will use this count the string words again...
    * we can maintain counter that will count the size of every word..
    * We will also maintains currStart variable that will maintains the starting position of every word while counting it..
    * at last we finally maintains minSize that will store the minimum size of word in string & minStartPos that will store the minimum word starting index..
    * we can iterate from 0(inclusive) to n(inclusive).
        * while iterating when (s[i] != ' ') cnt++, else cnt = 0 & store the starting position of shortest word...


// Example Testcases: 

// Input:
programming is lob
my name is sahil
open source contributor
is is is is is
sahilwep

// Output:
is
my
open
is
sahilwep



// Time Complexity: 
    O(N)

*/


use std::io::{self, Write};

fn shortest_word(s: &str) {
    let mut min_start_pos: isize = -1; // store starting index of shortest word
    let mut min_size = usize::MAX; // store the size of the shortest word
    let mut curr_start: isize = -1; // stores starting index of current word
    let mut cnt = 0; // counter to count each word size

    // iterate from 0 to s.len() inclusive, case when we only have one word in string
    for (i, c) in s.chars().chain(Some(' ')).enumerate() {
        if c != ' ' {
            if cnt == 0 {
                curr_start = i as isize; // store starting index of every word
            }
            cnt += 1; // increasing the counter
        } else {
            // when counter has counted some characters & it's lesser than the minSize we found earlier
            if cnt > 0 && cnt < min_size {
                min_size = cnt;
                min_start_pos = curr_start; // store curr_start into min_start_pos as we encountered smaller word
            }
            cnt = 0; // reset counter
        }
    }

    if min_start_pos == -1 {
        println!("Empty String, no words found!");
    } else {
        // we have starting index of smallest word and its size, so we can iterate to print it
        for i in min_start_pos..min_start_pos + min_size as isize {
            print!("{}", s.chars().nth(i as usize).unwrap());
        }
        println!();
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim();
    shortest_word(s);
}
