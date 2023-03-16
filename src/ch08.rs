pub fn main() {
    let emoji = "zaemon1251-hesty😀";
    println!("{}", emoji);

    let name = read_username_from_file().expect("fail to open");
    println!("{}", name);
}

use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
