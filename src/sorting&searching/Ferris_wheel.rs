// Time limit: 1.00 s
// Memory limit: 512 MB



// There are n children who want to go to a Ferris wheel, and your task is to find a gondola for each child.
// Each gondola may have one or two children in it, and in addition, the total weight in a gondola may not exceed x. You know the weight of every child.
// What is the minimum number of gondolas needed for the children?
// Input
// The first input line contains two integers n and x: the number of children and the maximum allowed weight.
// The next line contains n integers p_1,p_2,\ldots,p_n: the weight of each child.
// Output
// Print one integer: the minimum number of gondolas.
// Constraints

// 1 \le n \le 2 \cdot 10^5
// 1 \le x \le 10^9
// 1 \le p_i \le x

// Example
// Input:
// 4 10
// 7 2 3 9

// Output:
// 3
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<usize> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    let (n, maximum_weight) = (parts[0], parts[1] as i64);
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut weights: Vec<i64> = input.trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    
    weights.sort(); // Sort weights in ascending order
    
    let mut gondolas = 0;
    let mut left = 0;
    let mut right = n - 1;
    
    while left <= right {
        // If we can pair the heaviest with the lightest
        if left < right && weights[left] + weights[right] <= maximum_weight {
            left += 1;  // Move lightest pointer forward
            right -= 1; // Move heaviest pointer backward
        } else {
            // Just take the heaviest child alone
            right -= 1;
        }
        gondolas += 1;
    }
    
    println!("{}", gondolas);
}