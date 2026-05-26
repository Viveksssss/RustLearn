use std::{
    fs::{self, File},
    io::{Read, Write},
};

pub fn test1() -> Result<String, std::io::Error> {
    // let f = File::open("./hello.txt").unwrap();

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem openning the file :{:?}", error)
    //     }
    // };

    let f = std::fs::File::open("./hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn test2() -> Result<String, std::io::Error> {
    // let mut f = File::open("./hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)
    fs::read_to_string("./hello.txt") // 一步到位
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
