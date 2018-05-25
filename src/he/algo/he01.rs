/// https://www.hackerearth.com/practice/data-structures/arrays/1-d/practice-problems/algorithm/mark-the-answer-1/

use mylib::*;

pub fn mark_ans() {
    let mut array: Vec<i32> = Vec::new();
    let ((n, _, _), _) = readInt("i32");
    let ((x, _, _), _) = readInt("i32");
    for _ in 0..n {
        let ((tp, _, _), _) = readInt("i32");
        array.push(tp);
    }

    //problem solution
    let mut max_score = 0;
    let mut temp_score = 1;
    let mut skeep = 1;

    for val in array.iter() {
        if (*val < x) | ( skeep == 0 ) {
            temp_score += 1;
        } else {
            println!("temp score : {} ",temp_score);
            if temp_score > max_score { max_score = temp_score; }
            if skeep <= 0 {
                temp_score = 0;
            }
            skeep -=1;
        }
    }
    println!("{} ", max_score);
}