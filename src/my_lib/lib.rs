use std::io;

pub fn readInt(inp: &str) -> ((i32, i16, i8),(u32, u16, u8)) {
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
