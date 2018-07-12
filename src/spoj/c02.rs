
use mylib::*;

pub fn c02(){
    let (_,(_,_,test_cases)) = readInt("u8");
    let mut vec: Vec<i32> = Vec::new();
    readLine(&mut vec);
    let mut new_vec: Vec<i32> = Vec::new();
    for val in vec[0]..vec[1] + 1 {
        new_vec.push(val);
    }
   /* for x in vec[0]..vec[1] {
        //println!(" {} ",x);
        for v in 2..vec[1] {
            if v > vec[1]{break;}
            //new_vec.remove_item(&(x * v));
        }
    }*/

    for v in new_vec.iter() {
        print!("{} ," ,v);
    }

    new_vec.remove_item(&3);

    for v in new_vec.iter() {
        print!("{} ," ,v);
    }
}