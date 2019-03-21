extern crate uuid;
use self::uuid::Uuid;

use std::fs::File;
use std::fs;
use std::io::{Result, Write};

#[derive(Debug)]
pub struct Pair(pub String, pub String);

pub fn get_new_uuid() -> String {
    return format!("{}", Uuid::new_v4());
}

pub fn write_file(file_name: &String, file_string: &String) -> Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(file_string.as_bytes())?;
    Ok(())
}

pub fn read_file(file_name: &String) -> Result<String> {
    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");
    Ok(contents)
}
