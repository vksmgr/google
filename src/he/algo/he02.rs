// program to determine sting has unique chars.
use mylib::*;
pub fn unique_char(){
    let mut strng: Vec<char> = Vec::new();
    readString(&mut strng);
    println!("{:?}", strng);
}