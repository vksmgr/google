extern crate mylib;
use mylib::*;
use std::ascii::*;
use std::*;

//my modules
mod he;
mod spoj;

use std::io;
//imports from modules;
use spoj::*;

fn main(){
   // he02::unique_char();
    //c01::c01();
   // c02::c02();
    let mut ve: Vec<i32> = Vec::new();
    readLine(&mut ve);
}


/*
//change string this function will change the string char by char

fn add1_char(c: char) -> char{
    std::char::from_u32(c as u32 + 1).unwrap_or(c)
}

fn add1_str(s: &str) ->String{
    s.chars().map(add1_char).collect()
}*/
