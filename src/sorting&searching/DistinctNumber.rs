/*
Time limit: 1.00 s
Memory limit: 512 MB

You are given a list of n integers, and your task is to calculate the number of distinct values in the list.
Input
The first input line has an integer n: the number of values.
The second line has n integers x_1,x_2,\dots,x_n.
Output
Print one integers: the number of distinct values.
Constraints

1 <= n <= 2.10^5
1 <= x_i <= 10^9

Example
Input:
5
2 3 2 2 3

Output:
2
*/

use std::collections::HashSet;
use std::io;

fn main(){


    let mut n = String::new();
    io::stdin().read_line(& mut n).unwrap();
    let _n:usize = n.parse().unwrap();

    let mut value = String::new();
    io::stdin().read_line(& mut value).unwrap();

    let unique_values = value.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();


    println!("{}",unique_values.len());

}

