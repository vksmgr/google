extern crate mylib;
use mylib::*;

//my modules
mod he;

//imports from modules;
use he::algo::*;

fn main(){
    he02::unique_char();
}