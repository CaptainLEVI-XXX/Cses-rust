use std::io::{self};
use std::collections::BTreeMap;

fn main() {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts:Vec<i64> = input.trim().split_whitespace().map(|x|x.parse::<i64>().unwrap()).collect();
    
    let (_n,_m) = (parts[0],parts[1]);
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let tickets_price:Vec<i64> = input.trim().split_whitespace().map(|x|x.parse::<i64>().unwrap()).collect();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let max_prices:Vec<i64> = input.trim().split_whitespace().map(|x|x.parse::<i64>().unwrap()).collect();
    
    let mut tickets:BTreeMap<i64,i32> = BTreeMap::new();
    
    for  price in tickets_price{
        *tickets.entry(price).or_insert(0) +=1;
    }
    
    for max_price in max_prices {
        let mut result = -1;
        if let Some((&price,_)) = tickets.range(..=max_price).rev().next(){
            result =price;
            
            let count = tickets.get_mut(&price).unwrap();
            *count -=1;
            if *count == 0 {
                tickets.remove(&price);
            }
            
        }
         println!("{}", result);
        
    }
    
    
}