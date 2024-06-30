/*
//  Question: Implement a function that checks whether a given number is prime or not.

// Observations:
    * we just need to return wether the given number is prime or not.
    * Condition for number to be prime:
        * number should be only divisible by 1 & itself..
        * NOTE: 1 is not prime number...
        * prime numbers: 2, 3, 5, 7, 11, 13...

// Intrusion:
    * 1 is not prime
    * 2 & 3 is prime
    * Multiples of 2 & 3 are prime..
    * we can start finding from 5 to sqrt(n), because in b/w only the range we have prime numbers...
    * Example: 
        Prime Numbes: 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18....Sqrt(n)

        * we can eliminate the number that are divisible b/y 3 & 2 for large iteration of n.
        * By checking (n%2==0) & (n%3==0), we can save many iterations.
        * after removing from the list : we have to check : 
        
        5, 7, 11, 13, 17, 19,.....sqrt(n) 

        * we can start loop from 5 till sqrt(n) & increment with (i=i+6)
        * we only have to check this condition inside the loop.
                    if(n%i==0 || n%(i+2)==0) return false;

        * Example  :
                n = 121
                        i = 5
                        i = 11

                n = 1031
                        i = 5
                        i = 11
                        i = 17
                        i = 23
                        i = 29
                    Note : 35 * 35 = 1225, which is more than 1031.

        * (i*i  = sqrt(n)) equivalent, we can write..

// Example Test-cases: 
// Input: 
1
2
3
5
7
23
25

// Output: 
Not Prime
Prime
Prime
Prime
Prime
Prime
Not Prime


// Time Complexity: O(sqrt(n))

*/


use std::io;

fn is_prime(n: i32) -> bool {
    if n == 1 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Invalid input");

    if is_prime(n) {
        println!("Prime");
    } else {
        println!("Not Prime");
    }
}