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
    let parts: Vec<i64> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    let n = parts[0];
    let maximum_weight = parts[1];
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut weights: Vec<i64> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    weights.sort();
    
    let mut gondolas = 0;
    let mut left = 0;
    let mut right = n - 1;
    
    while left <= right {
        if left == right {
            // Only one child left, they get their own gondola
            gondolas += 1;
            break;
        }
        
        if weights[left as usize] + weights[right as usize] <= maximum_weight {
            // Can pair lightest with heaviest
            left += 1;
            right -= 1;
        } else {
            // Heaviest child must ride alone
            right -= 1;
        }
        
        gondolas += 1;
    }
    
    println!("{}", gondolas);
}