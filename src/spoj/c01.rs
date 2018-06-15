use mylib::*;

pub fn c01(){
    loop {
        let ((_,x,_),_) = readInt("i16");
        if x == 42 {
            break;
        }else {
            println!("{}", x);
        }

    }
}