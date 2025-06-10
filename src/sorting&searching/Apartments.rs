// Time limit: 1.00 s
// Memory limit: 512 MB



// There are n applicants and m free apartments. Your task is to distribute the apartments so that as many applicants as possible will get an apartment.
// Each applicant has a desired apartment size, and they will accept any apartment whose size is close enough to the desired size.
// Input
// The first input line has three integers n, m, and k: the number of applicants, the number of apartments, and the maximum allowed difference.
// The next line contains n integers a_1, a_2, \ldots, a_n: the desired apartment size of each applicant. If the desired size of an applicant is x, they will accept any apartment whose size is between x-k and x+k.
// The last line contains m integers b_1, b_2, \ldots, b_m: the size of each apartment.
// Output
// Print one integer: the number of applicants who will get an apartment.
// Constraints

// 1 \le n, m \le 2 \cdot 10^5
// 0 \le k \le 10^9
// 1 \le a_i, b_i \le 10^9

// Example
// Input:
// 4 3 5
// 60 45 80 60
// 30 60 75

// Output:
// 2

use std::io;
fn main(){


    let mut input= String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut parts = Vec::new();
    parts = input.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();

    let (n,m,k) = (parts[0],parts[1],parts[2] as i64);

    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let mut applicants:Vec<i64> = input.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();


    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let mut apartment:Vec<i64> = input.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

    applicants.sort();
    apartment.sort();

    let mut i=0;
    let mut j=0;
    let mut matches =0 ;


    while i<n && j<m{
        if apartment[j] < applicants[i]-k{
            j+=1;
        }else if apartment[j] <= applicants[i]+k{
            i+=1;
            j+=1;
            matches+=1;
        }else{
            i+=1;
        }

    }

    println!("{}",matches);

}
