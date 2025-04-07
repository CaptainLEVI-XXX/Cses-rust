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

use std::collections::HashMap;
use std::io;


fn main(){

    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n:usize = n.trim().parse().unwrap(); 

    let mut mp = HashMap::new();
    for _ in 0..n{

        let mut value = String::new();
        io::stdin().read_line(&mut value).unwrap();
        let value:usize = value.trim().parse().unwrap();
        if  !mp.contains_key(&value) {
            mp.insert(value,true);
  
        }
    }
    println!("Unique Values are : {}",mp.len())

    
}