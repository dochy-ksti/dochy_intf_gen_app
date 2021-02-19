use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf};
use std::io::Error;

pub fn write_file(contents: &str, path : &PathBuf) -> Result<String, Error>{
    {
        match File::open(path){
            Ok(mut file) => {
                if contents.len() as u64 == file.metadata()?.len() {
                    let mut buf: Vec<u8> = Vec::new();
                    file.read_to_end(&mut buf)?;
                    if contents.as_bytes() == buf.as_slice() {
                        return Ok("No need to update.".to_string());
                    }
                }
            },
            Err(_) =>{},
        }
    }

    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok("Updated successfully".to_string())
}