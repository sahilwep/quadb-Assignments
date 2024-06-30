/*
//  Question: Implement a function that finds the longest common prefix of a given set of strings.

// Observations: 
    * Longest common prefix of given set of string..
    * prefix means the starting characters of strings...
    * Idea is to sort the array, which will sort the array in alphabetic order of every character..
    * Then we can get the first & last string from the set of string..
    * then we can start matching the individual character in both the string...
    * till it's matched we will print or store, & when there is difference arises we break out..

// Intrusion: 
    * sort the array..
    * get the first & last sting..
    * Then we can find the common prefix b/w these two strings..
        * we can iterate b/w these two & at that these two are same, we will store that character into prefix string..
    * at last we return the prefix..


// Example Testcases: 
// Input: 
sahil sam saurabh saw
street stress string streak
moon mook moos mooj
aaak aaab aaac

// Ouput: 
sa
str
moo
aaa


// Time Complexity: 
    O(nlogn) + O(m) => O(nlogn)
        * nlogn => sorting
        * m => for finding the common prefix..


*/


fn longest_common_prefix(s: Vec<&str>) -> String {
    if s.is_empty() {
        return "-1".to_string(); // when there are no words in the string, we return "-1"
    }

    let mut s = s.clone(); // clone the input vector
    s.sort(); // sort the array of strings

    let first = &s[0]; // first index string
    let last = &s[s.len() - 1]; // last index string
    let min_length = first.len().min(last.len()); // minimum size among these two

    // Find the common prefix between the first and last string
    let mut i = 0;
    let mut prefix = String::new(); // result string that will store the prefix
    while i < min_length && first.as_bytes()[i] == last.as_bytes()[i] {
        prefix.push(first.chars().nth(i).unwrap()); // storing the prefix
        i += 1;
    }

    prefix // return the prefix string
}

fn main() {
    // example
    let s1 = vec!["sahil", "sam", "saurabh", "saw"];
    println!("{}", longest_common_prefix(s1));

    // ex: 2
    let s2 = vec!["street", "stress", "string", "streak"];
    println!("{}", longest_common_prefix(s2));

    // ex: 3
    let s3 = vec!["moon", "mook", "moos", "mooj"];
    println!("{}", longest_common_prefix(s3));

    // ex: 4
    let s4 = vec!["aaak", "aaab", "aaac"];
    println!("{}", longest_common_prefix(s4));
example