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
fn main(){

let mut input = String::new();
io::stdin().read_line(& mut input).unwrap();
let parts:Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

let (n,maximum_weight) = (parts[0],parts[1] as i64);

input.clear();
io::stdin().read_line(&mut input).unwrap();
let mut weights:Vec<i64>= input.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();


weights.sort();
// 4 10
// 7 2 3 9

//2 3 7 9


let mut i=0;
let mut gondola=0;
let mut sum=0;

while i<n{

sum += weights[i];
if sum > maximum_weight {
    gondola+=1;
    sum = weights[i];
}
i+=1;
}
if sum != 0 {gondola+=1};

println!("{}",gondola);


}