use std::io;

pub fn readInt(inp: &str) -> ((i32, i16, i8), (u32, u16, u8)) {
    let mut buff: String = String::new();
    io::stdin().read_line(&mut buff).expect("error while reading");
    let buff = buff.trim();

    (match inp {
        "i32" => (buff.parse::<i32>().unwrap(), 0, 0),
        "i16" => (0, buff.parse::<i16>().unwrap(), 0),
        "i8" => (0, 0, buff.parse::<i8>().unwrap()),
        _ => (0, 0, 0)
    },
     match inp {
         "u32" => (buff.parse::<u32>().unwrap(), 0, 0),
         "u16" => (0, buff.parse::<u16>().unwrap(), 0),
         "u8" => (0, 0, buff.parse::<u8>().unwrap()),
         _ => (0, 0, 0)
     })
}

//this method will read sting and return it in vec f char
pub fn readString(vec: &mut Vec<char>) {
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).expect("error while reading sting");
    let buff = buff.trim().to_string();
    for chars in buff.chars() {
        vec.push(chars);
    }
    //String::from_utf8(buff.into_bytes());
}

pub fn getASCII(ch: char) -> u8 {
    let alpha = [('a', 97), ('b', 98), ('c', 99), ('d', 100), ('e', 101), ('f', 102),
        ('g', 103), ('h', 104), ('i', 105), ('j', 106), ('k', 107), ('l', 108), ('m', 109), ('n', 110),
        ('o', 111), ('p', 112), ('q', 113), ('r', 114), ('s', 115), ('t', 116), ('u', 117), ('v', 118),
        ('w', 119), ('x', 120), ('y', 121), ('z', 122),];
    let x = alpha.iter().find(|t| t.0 == ch);
    let (t, i) = x.unwrap();
    //print!("value: {}",i);
    *i as u8
}