//https://www.hackerearth.com/practice/algorithms/string-algorithm/basics-of-string-manipulation/practice-problems/algorithm/marut-and-strings-4/
use mylib::*;
use std::io;
pub fn marut(){
    let (_,(_,_,T)) = readInt("u8");
    for _ in 0..T {
        solv();
    }
}

fn solv(){
    let mut strng = String::new();
    io::stdin().read_line(&mut strng).expect("Error");
    let strng = strng.trim().to_string();

    let mut count_upper: u16 = 0;let mut count_lower: u16 = 0;
    for char in strng.chars() {

        if char.is_alphanumeric() & char.is_uppercase() {
            count_upper +=1;
            //print!("upper : {} ",count_upper);
        }else if char.is_alphanumeric() & char.is_lowercase() {

            count_lower +=1;
            //print!("Lower : {} ",count_lower);

        }else {
            println!("Invalid Input");
            break;
        }
    }
    if count_lower < count_upper { println!("{}", count_lower); }
    else { println!("{}", count_upper); }
}