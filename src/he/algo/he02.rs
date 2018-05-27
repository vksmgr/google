// program to determine sting has unique chars.
use mylib::*;


pub fn unique_char(){
    let mut strng: Vec<char> = Vec::new();
    readString(&mut strng);
    let mut count: u8 = 0;
    let mut flag = false;
    for ch in strng.iter() {
        for xc in strng.iter() {
            if ch == xc { count += 1;}
            if count > 1{flag = true;break;}
        }

    }
    if flag { print!("No string does not have all unique chars"); }
    else { println!("String have all char as unique"); }
}