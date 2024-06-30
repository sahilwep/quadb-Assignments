/*
//  Question: Merge two sorted arrays in Rust

// Observations: 
    * As we have sorted array given, we just have to merge it & from a new array...

// Intrusion: 
    * We can use two pointer approach: 
        * first pointer in first array
        * Second pointer in second array..
    * We will compare each pointer element & whose is smaller, we will store in third array..
    * Last we have to look after the left over element, that are not stores..
        * we can check this in both the array...
    * and last we will print the final array..

// Example TestCases: 
// Input:
4 5
10 15 20 40
5 6 6 10 15

// Output: 
5 6 6 10 10 15 15 20 40 


// Time Complexity: 
    O(n + m), sum of size of first array & second array..


*/


// Merge two sorted arrays
fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let n = a.len();
    let m = b.len();
    let mut arr = vec![0; n + m];

    while i < n && j < m {
        if a[i] < b[j] {
            arr[k] = a[i];
            i += 1;
        } else {
            arr[k] = b[j];
            j += 1;
        }
        k += 1;
    }

    while i < n {
        arr[k] = a[i];
        i += 1;
        k += 1;
    }

    while j < m {
        arr[k] = b[j];
        j += 1;
        k += 1;
    }

    arr
}

fn main() {
    let a = [10, 15, 20, 40];
    let b = [5, 6, 6, 10, 15];
    
    let merged_array = merge(&a, &b);
    
    for num in &merged_array {
        print!("{} ", num);
    }
    println!();
}
