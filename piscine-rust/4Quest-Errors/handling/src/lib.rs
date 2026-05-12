use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {    
    let mut file = match OpenOptions::new().append(true).open(path) {
        Ok(file) => file,
        Err(error) => match error.kind() {           
            ErrorKind::NotFound => File::create(path).unwrap(),
            other_error => panic!("Failed to open file: {:?}", other_error),
        },
    };    
    file.write_all(content.as_bytes()).unwrap();
}