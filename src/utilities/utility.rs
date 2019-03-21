extern crate uuid;
use self::uuid::Uuid;

use std::fs::File;
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
